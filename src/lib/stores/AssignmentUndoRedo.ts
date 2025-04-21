import {
    getLocalAssignment,
    saveAssignment,
    deleteAssignment,
} from "$lib/modules/entities/assignments";


const MAX_STACK_SIZE = 10
export type actionType = "create" | "delete"

interface Assigments {
    groupId: number,
    day: string,
    action: actionType
    moduleIndex: number,
    subjectId: number,
    teacherId: number,
}

type assigmentMap = Map<string, Assigments>

const undoStack: assigmentMap[] = []
const redoStack: assigmentMap[] = []

function getkey(assignment: Assigments) {
    return `${assignment.groupId}-${assignment.day}-${assignment.moduleIndex}`
}

export function commitChange(newChange: Assigments) {
    if (undoStack.length > MAX_STACK_SIZE) undoStack.shift()
    const key = getkey(newChange)

    undoStack.push(new Map([[key, newChange]]))
    console.log("Undo stack", undoStack)
    redoStack.length = 0
}

export async function undoChange() {
    if (undoStack.length === 0) return

    const lastChange = undoStack.pop()
    if (!lastChange) return
    redoStack.push(lastChange)
    const [key, change] = Array.from(lastChange.entries())[0]

    if (change.action === "create") {
        const assignment = getLocalAssignment(change.groupId, change.day, change.moduleIndex)
        await deleteAssignment(assignment.id)
    } else if (change.action === "delete") {
        if (change.subjectId && change.teacherId) {
            await saveAssignment(
                change.groupId,
                change.day,
                change.moduleIndex,
                change.subjectId,
                change.teacherId,
            )
        }
    }
}

export async function redoChange() {
    if (redoStack.length === 0) return

    const lastChange = redoStack.pop()
    if (!lastChange) return
    undoStack.push(lastChange)
    const [key, change] = Array.from(lastChange.entries())[0];

    if(change.action === "create") {
        if(change.subjectId && change.teacherId) {
            await saveAssignment(
                change.groupId,
                change.day,
                change.moduleIndex,
                change.subjectId,
                change.teacherId,
            )
        }
    } else if (change.action === "delete") {
        const assignment = getLocalAssignment(change.groupId, change.day, change.moduleIndex)
        await deleteAssignment(assignment.id)
    }
}

export function findDropTarget(e: MouseEvent): HTMLElement | null {
    // Get all elements at the current mouse position
    const elements = document.elementsFromPoint(e.clientX, e.clientY);

    // Find the first element with class 'module-cell'
    for (const el of elements) {
        if (el.classList.contains("module-cell")) {
            return el as HTMLElement;
        }
    }
    return null;
}
