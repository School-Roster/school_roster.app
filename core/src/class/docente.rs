/*
   Este archivo contiene la definición de la estructura Docente.
   Cada docente tiene un id, un nombre, un apellido, un email, horas
   comisionadas, materias, color, rendimiento y rendimiento por materia.
*/

pub struct Docente
{
    id: i32,
    nombre: String,
    apellido: String,
    email: String,
    horas_com: i32,
    materias: String,
    color: String,
    rendimiento: u8, // 1-5 stars
    rendimiento_materia: Vec<u8>, // rendimiento por materia sera un vector de numeros del 1 al 5
}

/* Implementación de métodos para la estructura Docente. */
impl Docente
{
    pub fn new(id: i32, nombre: String, apellido: String, email: String, horas_com: i32, materias: String, color: String, rendimiento: u8, rendimiento_materia: Vec<u8>) -> Docente
    {
        Docente
        {
            id,
            nombre,
            apellido,
            email,
            horas_com,
            materias,
            color,
            rendimiento,
            rendimiento_materia,
        }
    }

    /* Metodos getter para la estructura Docente. */
    pub fn get_id(&self) -> i32
    {
        self.id
    }

    pub fn get_nombre(&self) -> String
    {
        self.nombre.clone()
    }

    pub fn get_apellido(&self) -> String
    {
        self.apellido.clone()
    }

    pub fn get_email(&self) -> String
    {
        self.email.clone()
    }

    pub fn get_horas_com(&self) -> i32
    {
        self.horas_com
    }

    pub fn get_materias(&self) -> String
    {
        self.materias.clone()
    }

    pub fn get_color(&self) -> String
    {
        self.color.clone()
    }

    pub fn get_rendimiento(&self) -> u8
    {
        self.rendimiento
    }

    pub fn get_rendimiento_materia(&self) -> Vec<u8>
    {
        self.rendimiento_materia.clone()
    }

    /* Metodos setter para la estructura Docente. */
    pub fn set_id(&mut self, id: i32)
    {
        self.id = id;
    }

    pub fn set_nombre(&mut self, nombre: String)
    {
        self.nombre = nombre;
    }

    pub fn set_apellido(&mut self, apellido: String)
    {
        self.apellido = apellido;
    }

    pub fn set_email(&mut self, email: String)
    {
        self.email = email;
    }

    pub fn set_horas_com(&mut self, horas_com: i32)
    {
        self.horas_com = horas_com;
    }

    pub fn set_materias(&mut self, materias: String)
    {
        self.materias = materias;
    }

    pub fn set_color(&mut self, color: String)
    {
        self.color = color;
    }

    pub fn set_rendimiento(&mut self, rendimiento: u8)
    {
        self.rendimiento = rendimiento;
    }

    // Este metodo recibira el vector con los rendimientos de cada materia
    pub fn set_rendimiento_materia(&mut self, rendimiento_materia: Vec<u8>)
    {
        self.rendimiento_materia = rendimiento_materia;
    }

    // Este metodo agregara un rendimiento de materia al vector ya existente
    pub fn add_rendimiento_materia(&mut self, rendimiento_materia: u8)
    {
        // Validar que el vector exista
        // if self.rendimiento_materia.is_empty()
        // {
        //     self.rendimiento_materia = Vec::new();
        // }
        self.rendimiento_materia.push(rendimiento_materia);
    }

    // Este metodo eliminara un rendimiento de materia del vector ya existente
    // TODO: Este metodo ira en el frontend
    pub fn remove_rendimiento_materia(&mut self, rendimiento_materia: u8)
    {
        let index = self.rendimiento_materia.iter().position(|&x| x == rendimiento_materia);
        // Si se encontro el rendimiento de materia, eliminarlo
        if let Some(i) = index
        {
            self.rendimiento_materia.remove(i);
        }
    }
}
