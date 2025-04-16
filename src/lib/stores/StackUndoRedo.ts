import { writable } from "svelte/store"

interface Assigments {
    groupId: number,
    day: string,
    moduleIndex: number,
    subjectId?: number,
    teacherId?: number,
}

const MAX_STACK_SIZE = 10

type assigmentStack = Assigments[]

function createStack(initial: assigmentStack) {

    const { subscribe, update } = writable<assigmentStack>(initial);

    const undoStack: assigmentStack[] = []
    const redoStack: assigmentStack[] = []

    function commitChanges(newState: assigmentStack) {
        update(current => {
            undoStack.push(structuredClone(current))
            if (undoStack.length > MAX_STACK_SIZE) undoStack.shift()
            redoStack.splice(0, redoStack.length)
            return newState
        })
    }


    function undo() {
        update(current => {
            if (undoStack.length === 0) return current

            redoStack.push(structuredClone(current))
            const previousState = undoStack.pop()

            console.log("Undo ejecutado")
            console.log("Nuevo estado actual:", previousState)
            console.log("<-- undoStack:", structuredClone(undoStack))
            console.log("--> redoStack:", structuredClone(redoStack))


            return previousState ? previousState : current
        })
    }

    function redo() {
        update(current => {
            if (redoStack.length === 0) return current

            undoStack.push(structuredClone(current))
            const nextState = redoStack.pop()

            console.log("Redo ejecutado")
            console.log("Nuevo estado actual:", nextState)
            console.log("<-- undoStack:", structuredClone(undoStack))
            console.log("--> redoStack:", structuredClone(redoStack))

            return nextState ? nextState : current
        })
    }


    return {
        subscribe,
        commitChanges,
        undo,
        redo
    }
}

export const UndoRedoStore = createStack([])

