pub mod table {
    #![allow(dead_code, non_snake_case)]
    use rand::distributions::{Distribution, Uniform};
    use rusqlite::{params};

    const CHARS: [&'static str; 52] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    fn get_ascii(c: usize) -> &'static str {
        CHARS.get(c as usize).unwrap_or(&"")
    }
    fn fill_str() -> String {
        let between = Uniform::from(0..52);
        let mut rng = rand::thread_rng();
        let mut vector = vec![];
        for _ in 0..10 {
            vector.push(get_ascii(between.sample(&mut rng)).to_string())
        }
        vector.into_iter().collect::<String>()
    }
    pub fn fill_vec_str(size: i32) -> Vec<String> {
        let mut vec = vec![];
        for _ in 0..size {
            vec.push(fill_str());
        }
        vec
    }

    pub fn insert_role (con: &rusqlite::Connection, size: usize) {
        for _ in 0..size {
            let rollee = fill_vec_str(3);
            con.execute(
                "INSERT INTO role VALUES (?1, ?2, ?3);",
                params![rollee[0], rollee[1], rollee[2]],
            ).unwrap();
        }
    }

    pub mod get_tables {
        #[derive(Debug)]
        pub struct User {
            pub Firstname: String,
            pub Surname: String,
            pub YearGroup: String,
            pub Middlename: String,
            pub Preferredname: String,
            pub Username: String,
            pub Status: String,
            pub Year: i32,
            pub Email: String,
            pub SchoolID: String,
            pub IsamsID: String,
            pub IsamsCode: String,
            pub Title: String,
            pub LastActive: String,
            pub PupilType: String,
            pub AcademicStudy: String,
            pub Positions: String,
            pub ExpoID: String,
            pub Archived: i32,
            pub House: String,
            pub Roles: String,
        }
        #[derive(Debug)]
        pub struct Role {
            pub name: String,
            pub auth: String,
            pub typeS: String,
        }
    }

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    pub fn get_ddl(path: String) -> impl Iterator<Item = String> {
        BufReader::new(File::open(path).unwrap())
            .lines()
            .map(Result::unwrap)
    }
}
