use crate::db;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Subject
{
    pub id: i16,
    pub name: String,
    pub shorten: String,
    pub color: String,
    pub stype: String,
}

impl Subject
{
    /* Constructor de la estructura Grupo. */
    pub fn new(id: i16, nombre: String, abreviacion: String, color: String, tipo: String) -> Subject
    {
        Self
        {
            id,
            name: nombre,
            shorten: abreviacion,
            stype: tipo,
            color,
        }
    }
}

pub fn get_all() -> Vec<Subject>
{
    let query = "SELECT * FROM subjects";
    let conn = match db::connect() {
        Ok(conn) => conn,
        Err(e) => panic!("Error connecting to the database: {}", e),
    };

    let mut subjects: Vec<Subject> = Vec::new();
    conn.iterate(query, |pairs| {
        let mut subject: Subject = Subject {
            id: 0,
            name: String::from(""),
            shorten: String::from(""),
            color: String::from(""),
            stype: String::from(""),
        };

        for &(column, value) in pairs.iter() {
            match column {
                "id" => subject.id = value.unwrap().parse::<i16>().unwrap(),
                "name" => subject.name = String::from(value.unwrap()),
                "shorten" => subject.shorten = String::from(value.unwrap()),
                "color" => subject.color = String::from(value.unwrap()),
                "type" => subject.stype = String::from(value.unwrap_or("")),
                _ => (),
            }
        }

        subjects.push(subject);
        true
    }).unwrap();

    subjects
}

pub fn register(name: &str, shorten: &str, color: &str, _type: &str) -> String
{
    println!("{}", _type);
    let conn = match db::connect() {
        Ok(conn) => conn,
        Err(e) => panic!("Error connecting to the database: {}", e),
    };

    let query = format!(
        "INSERT INTO subjects (name, shorten, color, type) VALUES ('{}', '{}', '{}', '{}')",
        name, shorten, color, _type
    );

    match conn.execute(&query) {
        // Ok(_) => println!("Subject registered successfully!"),
        Ok(_) => return String::from("Subject registered successfully!"),
        Err(e) => {
            return format!("Error registering the subject");
        }
    }
}
