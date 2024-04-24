/*
   Este archivo contiene la definición de la estructura para los Grupos.
   Cada grupo tiene un id
*/

pub struct Grupo
{
    pub id: i16,
    pub grado: i16,
    pub grupo: String,
    pub carrera: String, // Carrera solo sera activado si el grupo es de licenciatura o ingenieria/tecnologo
    pub turno: String,
    pub aula: String, // Aula solo sera activado si el grupo tiene una aula asignada
    pub num_alumnos: i32,
}

pub enum GrupoError
{
    InvalidId,
    InvalidGrado,
    InvalidGrupo,
    InvalidCarrera,
    InvalidTurno,
    InvalidAula,
    InvalidNumAlumnos,
}

/* Implementación de métodos para la estructura Grupo. */
pub impl Grupo
{
    /* Constructor de la estructura Grupo. */
    pub fn new(id: i16, grado: i16, grupo: String, carrera: String, turno: String, aula: String, num_alumnos: i32) -> Grupo
    {
        Grupo
        {
            id,
            grado,
            grupo,
            carrera,
            turno,
            aula,
            num_alumnos
        }
    }

    /* Metodos getter para la estructura Grupo. */
    pub fn get_id(&self) -> i16
    {
        self.id
    }

    pub fn get_grado(&self) -> i16
    {
        self.grado
    }

    pub fn get_grupo(&self) -> String
    {
        self.grupo.clone()
    }

    pub fn get_carrera(&self) -> String
    {
        self.carrera.clone()
    }

    pub fn get_turno(&self) -> String
    {
        self.turno.clone()
    }

    pub fn get_aula(&self) -> String
    {
        self.aula.clone()
    }

    pub fn get_num_alumnos(&self) -> i32
    {
        self.num_alumnos
    }
}

/* Implementación de métodos para la estructura GrupoError. */
pub impl GrupoError
{
    pub fn description(&self) -> &str
    {
        match *self
        {
            GrupoError::InvalidId => "El id del grupo es invalido.",
            GrupoError::InvalidGrado => "El grado del grupo es invalido.",
            GrupoError::InvalidGrupo => "El grupo es invalido.",
            GrupoError::InvalidCarrera => "La carrera del grupo es invalida.",
            GrupoError::InvalidTurno => "El turno del grupo es invalido.",
            GrupoError::InvalidAula => "El aula del grupo es invalida.",
            GrupoError::InvalidNumAlumnos => "El numero de alumnos del grupo es invalido.",
        }
    }
}
