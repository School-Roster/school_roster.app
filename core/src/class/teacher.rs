use crate::db;

/* Struct that will hold every teacher's information */
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Teacher {
    pub id: i32,
    pub shorten: String, // If is None then will provide a default value
    pub name: String,
    pub first_last_name: String,
    pub second_last_name: String,

    // pub color: String,

    /* Optional fields */
    pub email: Option<String>,
    pub phone: Option<String>,
    pub degree: Option<String>,

    pub commissioned_hours: i32,
    pub active_hours: i32,
    pub general_stars: u8, // 0 to 5 (can be a float)
}

impl Teacher {
    pub fn new(
        id: i32,
        shorten: Option<String>,
        name: String,
        father_last_name: String,
        mother_last_name: String,
        email: Option<String>,
        phone: Option<String>,
        degrees: Option<String>,
        commissioned_hours: i32,
        active_hours: i32,
        general_stars: u8,
    ) -> Teacher {
        let shorten = match shorten {
            Some(shorten) => shorten,
            // None -> Example: Victor Rodriguez: VR
            None => String::from(format!("{}{}", name.chars().next().unwrap(), father_last_name.chars().next().unwrap())),
        };
        Teacher {
            id,
            shorten,
            name,
            first_last_name: father_last_name,
            second_last_name: mother_last_name,
            email,
            phone,
            degree: degrees,
            commissioned_hours,
            active_hours,
            general_stars,
        }
    }

    /* pub fn register_teacher(
        shorten: String,
        name: String,
        first_last_name: String,
        second_last_name: String,
        email: Option<String>,
        phone: Option<String>,
        degrees: Option<String>,
        commissioned_hours: i32,
        active_hours: i32,
        general_stars: u8,
    ) {
        let query = format!(
            "INSERT INTO teachers (shorten, name, first_last_name, second_last_name, email, phone, degrees, commissioned_hours, active_hours, general_stars) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, {}, {})",
            shorten, name, first_last_name, second_last_name, email.unwrap_or("".to_string()), phone.unwrap_or("".to_string()), degrees.unwrap_or("".to_string()), commissioned_hours, active_hours, general_stars
        );
        let conn: sqlite::Connection = db::connect().expect("Error connecting to the database");
        conn.execute(query);
    } */

    /* fn get_teacher_by_id(id: i32) -> Teacher {
        let query = format!("SELECT * FROM teachers WHERE id = {}", id);
        let conn: sqlite::Connection = db::connect();

        let mut teacher: Teacher;
        conn.iterate(query, |row| {
            let id: i32 = row.get(0);
            let shorten: String = row.get(1);
            let name: String = row.get(2);
            let first_last_name: String = row.get(3);
            let second_last_name: String = row.get(4);
            let email: Option<String> = row.get(5);
            let phone: Option<String> = row.get(6);
            let degrees: Option<String> = row.get(7);
            let commissioned_hours: i32 = row.get(8);
            let active_hours: i32 = row.get(9);
            let general_stars: u8 = row.get(10);

            teacher = Teacher::new(
                id,
                Some(shorten),
                name,
                first_last_name,
                second_last_name,
                email,
                phone,
                degrees,
                commissioned_hours,
                active_hours,
                general_stars,
            );
            true
        });
    }

    fn get_teacher_by_name(name: String) -> Teacher {
        let query = format!("SELECT * FROM teachers WHERE name = {}", name);
        let conn: sqlite::Connection = db::connect();

        let mut teacher: Teacher;
        conn.iterate(query, |row| {
            let id: i32 = row.get(0);
            let shorten: String = row.get(1);
            let name: String = row.get(2);
            let first_last_name: String = row.get(3);
            let second_last_name: String = row.get(4);
            let email: Option<String> = row.get(5);
            let phone: Option<String> = row.get(6);
            let degrees: Option<String> = row.get(7);
            let commissioned_hours: i32 = row.get(8);
            let active_hours: i32 = row.get(9);
            let general_stars: u8 = row.get(10);

            teacher = Teacher::new(
                id,
                Some(shorten),
                name,
                first_last_name,
                second_last_name,
                email,
                phone,
                degrees,
                commissioned_hours,
                active_hours,
                general_stars,
            );
            true
        });
    } */
}

pub fn get_teachers() -> Vec<Teacher> {
    let query = String::from("SELECT * FROM teachers");
    let conn = match db::connect() {
        Ok(conn) => conn,
        Err(e) => panic!("Error connecting to the database: {}", e),
    };

    let mut teachers: Vec<Teacher> = Vec::new();
    conn.iterate(query, |pairs| {
        let mut teacher: Teacher = Teacher {
            id: 0,
            shorten: String::from(""),
            name: String::from(""),
            first_last_name: String::from(""),
            second_last_name: String::from(""),
            email: None,
            phone: None,
            degree: None,
            commissioned_hours: 0,
            active_hours: 0,
            general_stars: 0,
        };

        for &(column, value) in pairs.iter() {
            match column {
                "id" => teacher.id = value.unwrap().parse::<i32>().unwrap(),
                "shorten" => teacher.shorten = String::from(value.unwrap()),
                "name" => teacher.name = String::from(value.unwrap()),
                "first_last_name" => teacher.first_last_name = String::from(value.unwrap()),
                "second_last_name" => teacher.second_last_name = String::from(value.unwrap()),
                "email" => teacher.email = Some(String::from(value.unwrap())),
                "phone" => teacher.phone = Some(String::from(value.unwrap())),
                "degrees" => teacher.degree = Some(String::from(value.unwrap())),
                "commissioned_hours" => teacher.commissioned_hours = value.unwrap().parse::<i32>().unwrap(),
                "active_hours" => teacher.active_hours = value.unwrap().parse::<i32>().unwrap(),
                "general_stars" => teacher.general_stars = value.unwrap().parse::<u8>().unwrap(),
                _ => (),
            }
        }

        teachers.push(teacher);
        true
    }).unwrap();

    // Delete 2 teachers to avoid duplicates
    // conn.execute("DELETE FROM teachers WHERE shorten = 'VR'").unwrap();
    // conn.execute("DELETE FROM teachers WHERE shorten = 'MM'").unwrap();

    // Register 2 teachers for testing purposes
    // test_register(&conn);

    teachers
}

// Register 2 teachers for testing purposes
// fn test_register(conn: &sqlite::Connection) {
//     conn.execute("
//         INSERT INTO
//         teachers (shorten, name, first_last_name, second_last_name, email, phone, degree, commissioned_hours, active_hours, general_stars)
//         VALUES ('VR', 'Victor', 'Rodriguez', 'Barragan', 'a21110120@ceti.mx', '', 'Ingenieria en Desarrollo de Software', 40, 40, 5)
//     ").unwrap();
//
//     conn.execute("
//         INSERT INTO
//         teachers (shorten, name, first_last_name, second_last_name, email, phone, degree, commissioned_hours, active_hours, general_stars)
//         VALUES ('MM', 'Moises', 'Morales', 'Lozano', 'a21110111@ceti.mx', '', 'Ingenieria en Desarrollo de Software', 40, 40, 5)
//     ").unwrap();
// }

pub fn register_teacher(
    shorten: String,
    name: String,
    first_last_name: String,
    second_last_name: String,
    email: Option<String>,
    phone: Option<String>,
    degree: Option<String>,
    commissioned_hours: i32,
    active_hours: i32,
    general_stars: u8,
) {
    let shorten = if shorten.is_empty() {
        format!("{}{}", name.chars().next().unwrap(), first_last_name.chars().next().unwrap())
    } else {
        shorten
    };
    let query = format!(
        "INSERT INTO
        teachers (
            shorten,
            name,
            first_last_name,
            second_last_name,
            email,
            phone,
            degree,
            commissioned_hours,
            active_hours,
            general_stars
        )
        VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, {}, {})",
        shorten, name, first_last_name, second_last_name, email.unwrap_or("".to_string()), phone.unwrap_or("".to_string()), degree.unwrap_or("".to_string()), commissioned_hours, active_hours, general_stars
    );
    let conn: sqlite::Connection = db::connect().expect("Error connecting to the database");
    conn.execute(query).unwrap();
}
