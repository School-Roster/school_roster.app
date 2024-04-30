/*
   Este archivo contiene la definición de la estructura para las materias.
   Cada materia tiene un id, nombre, abreviacion, tipo, color
*/

pub struct Materia
{
    pub id: i16,
    pub nombre: String,
    pub abreviacion: String,
    pub tipo: String,
    pub color: String
}

pub enum MateriaError
{
    InvalidId,
    InvalidNombre,
    InvalidAbreviacion,
    InvalidTipo,
    InvalidColor,
}

/* Implementación de métodos para la estructura Grupo. */
pub impl Materia
{
    /* Constructor de la estructura Grupo. */
    pub fn new(id: i16, nombre: String, abreviacion: String, tipo: String, color: String) -> Grupo
    {
        Grupo
        {
            id,
            nombre,
            abreviacion,
            tipo,
            color,
        }
    }

    /* Metodos getter para la estructura Grupo. */
    pub fn get_id(&self) -> i16
    {
        self.id
    }

    pub fn get_nombre(&self) -> String
    {
        self.nombre
    }

    pub fn get_abreviacion(&self) -> String
    {
        self.abreviacion
    }

    pub fn get_tipo(&self) -> String
    {
        self.tipo
    }

    pub fn get_color(&self) -> String
    {
        self.color
    }

    /* Metodos setter */
    pub fn set_id(&mut self, id: i16)
    {
        self.id = id;
    }

    pub fn set_nombre(&mut self, nombre: String)
    {
        self.nombre = nombre;
    }

}

/* Implementación de métodos para la estructura GrupoError. */
pub impl MateriaError
{
    pub fn description(&self) -> &str
    {
        match *self
        {
            MateriaError::InvalidId => "El id del grupo es invalido.",
            MateriaError::InvalidNombre => "El nombre de la materia es invalido.",
            MateriaError::InvalidAbreviacion => "Abreviacion invalida.",
            MateriaError::InvalidTipo => "El tipo de materia no es valido.",
            MateriaError::InvalidColor => "El color definido no existe.",
        }
    }
}
