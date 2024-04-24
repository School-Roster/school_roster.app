/*
    Este archivo contiene la definición de la estructura Aula, la cual
    representa un aula de la universidad. Cada aula tiene un id, un número
    de salón, un edificio, un tipo y una capacidad.
*/

pub struct Aula
{
    pub id: i32,
    pub salon: String,
    pub edificio: String,
    pub tipo: String,
    pub capacidad: i32
}

pub enum AulaError
{
    InvalidId,
    InvalidSalon,
    InvalidEdificio,
    InvalidTipo,
    InvalidCapacidad
}

/* Implementación de métodos para la estructura Aula. */
impl Aula
{
    /* Constructor de la estructura Aula. */
    pub fn new(id: i32, salon: String, edificio: String, tipo: String, capacidad: i32) -> Aula
    {
        Aula
        {
            id,
            salon,
            edificio,
            tipo,
            capacidad
        }
    }

    /* Metodos getter para la estructura Aula. */
    pub fn get_id(&self) -> i32
    {
        self.id
    }

    pub fn get_salon(&self) -> String
    {
        self.salon.clone()
    }

    pub fn get_edificio(&self) -> String
    {
        self.edificio.clone()
    }

    pub fn get_tipo(&self) -> String
    {
        self.tipo.clone()
    }

    pub fn get_capacidad(&self) -> i32
    {
        self.capacidad
    }

    /* Metodos setter para la estructura Aula. */
    pub fn set_id(&mut self, id: i32)
    {
        self.id = id;
    }

    pub fn set_salon(&mut self, salon: String)
    {
        self.salon = salon;
    }

    pub fn set_edificio(&mut self, edificio: String)
    {
        self.edificio = edificio;
    }

    pub fn set_tipo(&mut self, tipo: String)
    {
        self.tipo = tipo;
    }

    pub fn set_capacidad(&mut self, capacidad: i32)
    {
        self.capacidad = capacidad;
    }

    /* Método de validación de la estructura Aula. */
    /* TODO: este metodo ira en el frontend */
    pub fn validar(&self) -> bool
    {
        if self.salon.is_empty() || self.edificio.is_empty() || self.tipo.is_empty() || self.capacidad <= 0
        {
            return false;
        }
        true
    }
}

/* Implementación de métodos para la estructura AulaError. */
impl AulaError
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            AulaError::InvalidId => "El id del aula es inválido.".to_string(),
            AulaError::InvalidSalon => "El número de salón del aula es inválido.".to_string(),
            AulaError::InvalidEdificio => "El edificio del aula es inválido.".to_string(),
            AulaError::InvalidTipo => "El tipo de aula es inválido.".to_string(),
            AulaError::InvalidCapacidad => "La capacidad del aula es inválida.".to_string()
        }
    }
}
