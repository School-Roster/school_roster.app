import { 
    getLocalAssignment,
    saveAssignment,
    deleteAssignment 
} from "$lib/modules/entities/assignments";
import { invoke } from "@tauri-apps/api";

const MAX_STACK_SIZE = 10

export type actionType = "create" | "delete"

interface Assigments {
    groupId: number,
    day: string,
    action: actionType
    moduleIndex: number,
    subjectId?: number,
    teacherId?: number,
}

type assigmentMap = Map<string, Assigments>

const undoStack: assigmentMap[] = []
const redoStack: assigmentMap[] = []

function getkey(assignment: Assigments) {
    return `${assignment.groupId}-${assignment.day}-${assignment.moduleIndex}`
}

export function commitChange(newChange: Assigments) {
    if(undoStack.length > MAX_STACK_SIZE) undoStack.shift()
    const key = getkey(newChange)

    undoStack.push(new Map([[key, newChange]]))
    redoStack.length = 0
}

export async function undoChange() {
    if(undoStack.length === 0) return
    
    const lastChange = undoStack.pop()
    if(!lastChange) return
    redoStack.push(lastChange)
}

export async function redoChange() {
    if(redoStack.length === 0) return
    
    const lastChange = redoStack.pop()
    if(!lastChange) return
    undoStack.push(lastChange)
}
