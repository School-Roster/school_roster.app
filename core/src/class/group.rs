use crate::db;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub id: i32,
    pub grade: u8,
    pub group: String,
    pub spec: Option<String>,
    pub shift: String,
    pub students_count: i32,
}

impl Group {
    #[allow(unused)]
    fn new(id: i32, grade: u8, group: String, spec: Option<String>, shift: String, students_count: i32) -> Group {
        Self {
            id,
            grade,
            group,
            spec,
            shift,
            students_count,
        }
    }
}

pub fn get_groups() -> Vec<Group> {
    let query = "SELECT * FROM groups";
    let conn = match db::connect() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
            return Vec::new();
        }
    };

    let mut groups: Vec<Group> = Vec::new();
    conn.iterate(query, |row| {
        let mut group: Group = Group {
            id: 0,
            grade: 0,
            group: String::from(""),
            spec: None,
            shift: String::from(""),
            students_count: 0,
        };

        for &(column, value) in row.iter() {
            match column {
                "id" => group.id = value.unwrap().parse::<i32>().unwrap(),
                "grade" => group.grade = value.unwrap().parse::<u8>().unwrap(),
                "group_name" => group.group = String::from(value.unwrap()),
                "specialty" => group.spec = Some(String::from(value.unwrap())),
                "shift" => group.shift = String::from(value.unwrap()),
                "students_count" => group.students_count = value.unwrap().parse::<i32>().unwrap(),
                _ => (),
            }
        }

        groups.push(group);
        true
    }).unwrap();

    groups
}

pub fn register(grade: u8, group: String, spec: Option<String>, shift: String, students_count: i32) -> bool {
    let query = format!(
        "INSERT INTO groups (grade, group_name, specialty, shift, students_count) VALUES ({}, '{}', '{}', '{}', {})",
        grade,
        group,
        spec.unwrap_or(String::from("")),
        shift,
        students_count,
    );
    let conn = match db::connect() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
            return false;
        }
    };

    conn.execute(&query).unwrap();

    true
}
