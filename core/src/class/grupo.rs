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

    /* Metodos setter para la estructura Grupo. */
    pub fn set_id(&mut self, id: i16)
    {
        self.id = id;
    }

    pub fn set_grado(&mut self, grado: i16)
    {
        self.grado = grado;
    }

    pub fn set_grupo(&mut self, grupo: String)
    {
        self.grupo = grupo;
    }

    pub fn set_carrera(&mut self, carrera: String)
    {
        self.carrera = carrera;
    }

    pub fn set_turno(&mut self, turno: String)
    {
        self.turno = turno;
    }

    pub fn set_aula(&mut self, aula: String)
    {
        self.aula = aula;
    }

    pub fn set_num_alumnos(&mut self, num_alumnos: i32)
    {
        self.num_alumnos = num_alumnos;
    }

    /* Metodo para validar los datos de un grupo. */
    pub fn validate(&self) -> Result<(), GrupoError>
    {
        if self.id < 0
        {
            return Err(GrupoError::InvalidId);
        }
        if self.grado < 0
        {
            return Err(GrupoError::InvalidGrado);
        }
        if self.grupo.is_empty()
        {
            return Err(GrupoError::InvalidGrupo);
        }
        if self.carrera.is_empty()
        {
            return Err(GrupoError::InvalidCarrera);
        }
        if self.turno.is_empty()
        {
            return Err(GrupoError::InvalidTurno);
        }
        if self.aula.is_empty()
        {
            return Err(GrupoError::InvalidAula);
        }
        if self.num_alumnos < 0
        {
            return Err(GrupoError::InvalidNumAlumnos);
        }
        Ok(())
    }

    /* Database methods */
    pub fn save(&self) -> Result<(), GrupoError>
    {
        let conn = db::get_conn();
        let mut stmt = conn.prepare(
            "INSERT INTO grupo (id, grado, grupo, carrera, turno, aula, num_alumnos) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )?;
        stmt.execute(&[&self.id, &self.grado, &self.grupo, &self.carrera, &self.turno, &self.aula, &self.num_alumnos])?;
        Ok(())
    }

    pub fn update(&self) -> Result<(), GrupoError>
    {
        let conn = db::get_conn();
        let mut stmt = conn.prepare(
            "UPDATE grupo SET grado = ?, grupo = ?, carrera = ?, turno = ?, aula = ?, num_alumnos = ? WHERE id = ?"
        )?;
        stmt.execute(&[&self.grado, &self.grupo, &self.carrera, &self.turno, &self.aula, &self.num_alumnos, &self.id])?;
        Ok(())
    }

    pub fn delete(&self) -> Result<(), GrupoError>
    {
        let conn = db::get_conn();
        let mut stmt = conn.prepare("DELETE FROM grupo WHERE id = ?")?;
        stmt.execute(&[&self.id])?;
        Ok(())
    }

    pub fn get_all() -> Result<Vec<Grupo>, GrupoError>
    {
        let conn = db::get_conn();
        let mut stmt = conn.prepare("SELECT * FROM grupo")?;
        let grupos_iter = stmt.query_map(&[], |row| {
            Ok(Grupo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })?;

        let mut grupos = Vec::new();
        for grupo in grupos_iter
        {
            grupos.push(grupo?);
        }

        Ok(grupos)
    }

    pub fn get_by_id(id: i16) -> Result<Grupo, GrupoError>
    {
        let conn = db::get_conn();
        let mut stmt = conn.prepare("SELECT * FROM grupo WHERE id = ?")?;
        let grupo_iter = stmt.query_map(&[&id], |row| {
            Ok(Grupo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })?;

        let mut grupo = Grupo::new(0, 0, "".to_string(), "".to_string(), "".to_string(), "".to_string(), 0);
        for g in grupo_iter
        {
            grupo = g?;
        }

        Ok(grupo)
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
