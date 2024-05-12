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

    pub fn set_abreviacion(&mut self, abreviacion: String)
    {
        self.abreviacion = abreviacion;
    }

    pub fn set_tipo(&mut self, tipo: String)
    {
        self.tipo = tipo;
    }

    pub fn set_color(&mut self, color: String)
    {
        self.color = color;
    }

    /* Metodo para validar los datos de la materia. */
    pub fn validate(&self) -> Result<(), MateriaError>
    {
        if self.id < 0
        {
            return Err(MateriaError::InvalidId);
        }

        if self.nombre.is_empty()
        {
            return Err(MateriaError::InvalidNombre);
        }

        if self.abreviacion.is_empty()
        {
            return Err(MateriaError::InvalidAbreviacion);
        }

        if self.tipo.is_empty()
        {
            return Err(MateriaError::InvalidTipo);
        }

        if self.color.is_empty()
        {
            return Err(MateriaError::InvalidColor);
        }

        Ok(())
    }

    /* Database methods */
    pub fn save(&self) -> Result<(), MateriaError>
    {
        let conn = get_conn();
        let mut stmt = conn.prepare(
            "INSERT INTO materias (id, nombre, abreviacion, tipo, color) VALUES (?, ?, ?, ?, ?)"
        )?;
        stmt.execute(&[&self.id, &self.nombre, &self.abreviacion, &self.tipo, &self.color])?;
        Ok(())
    }

    pub fn update(&self) -> Result<(), MateriaError>
    {
        let conn = get_conn();
        let mut stmt = conn.prepare("UPDATE materias SET nombre = ?, abreviacion = ?, tipo = ?, color = ? WHERE id = ?")?;
        stmt.execute(&[&self.nombre, &self.abreviacion, &self.tipo, &self.color, &self.id])?;
        Ok(())
    }

    pub fn delete(&self) -> Result<(), MateriaError>
    {
        let conn = get_conn();
        let mut stmt = conn.prepare("DELETE FROM materias WHERE id = ?")?;
        stmt.execute(&[&self.id])?;
        Ok(())
    }

    pub fn get_all() -> Result<Vec<Materia>, MateriaError>
    {
        let conn = get_conn();
        let mut stmt = conn.prepare("SELECT * FROM materias")?;
        let materias_iter = stmt.query_map(&[], |row| {
            Ok(Materia::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut materias = Vec::new();
        for materia in materias_iter
        {
            materias.push(materia?);
        }

        Ok(materias)
    }

    pub fn get_by_id(id: i16) -> Result<Materia, MateriaError>
    {
        let conn = get_conn();
        let mut stmt = conn.prepare("SELECT * FROM materias WHERE id = ?")?;
        let materia_iter = stmt.query_map(&[&id], |row| {
            Ok(Materia::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut materias = Vec::new();
        for materia in materia_iter
        {
            materias.push(materia?);
        }

        if materias.len() == 0
        {
            Err(MateriaError::InvalidId)
        }
        else
        {
            Ok(materias[0].clone())
        }
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
