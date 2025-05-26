import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs"

// Se simulan los datos de la BD
const horarios = 
[
    {
        id: 1,
        grupo: 'A',
        materias: 'Matemáticas',
        profesor: 'Profesor Jirafales',
        inicio: '08:00',
        fin: '09:30',
        dia: 'Lunes'
    },

    {
        id: 1,
        grupo: 'A',
        materias: 'Español',
        profesor: 'Profesor Rafael',
        inicio: '08:00',
        fin: '09:30',
        dia: 'Martes'
    },

    {
        id: 2,
        grupo: 'B',
        materias: 'Matemáticas',
        profesor: 'Profesor Jirafales',
        inicio: '9:30',
        fin: '11:30',
        dia: 'Lunes'
    },

    {
        id: 2,
        grupo: 'B',
        materias: 'Español',
        profesor: 'Profesor Rafael',
        inicio: '09:30',
        fin: '11:30',
        dia: 'Martes'
    }
]


async function toJSON() {
    try {
        const json = JSON.stringify(horarios, null, 2)
        await writeTextFile("horarios.json", json, {dir: BaseDirectory.Download})
    } catch (error) {
        console.log(error)
    }
}

