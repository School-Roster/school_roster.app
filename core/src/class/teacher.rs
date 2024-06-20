use crate::db;

/* Struct that will hold every teacher's information */
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
    pub degrees: Option<String>,

    pub commissioned_hours: i32,
    pub active_hours: i32,
    pub general_stars: u8, // 0 to 5
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
            None => String::from("{}{}", name.chars().next().unwrap(), father_last_name.chars().next().unwrap()),
        };
        Teacher {
            id,
            shorten,
            name,
            first_last_name: father_last_name,
            second_last_name: mother_last_name,
            email,
            phone,
            degrees,
            commissioned_hours,
            active_hours,
            general_stars,
        }
    }

    pub fn register_teacher(
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
        let conn: sqlite::Connection = db::connect();
        conn.execute(query);
    }

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
    let conn: sqlite::Connection = db::connect();

    let mut teachers: Vec<Teacher> = Vec::new();
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

        teachers.push(Teacher::new(
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
        ));
        true
    });
}
    
