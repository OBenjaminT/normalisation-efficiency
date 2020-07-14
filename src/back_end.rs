pub mod table {
    #![allow(dead_code, non_snake_case)]
    use rusqlite::{params, Connection};

    pub fn create_table(conn: &Connection, set: schema::Table) {
        conn.execute(&db_meta::format_table(set)[..], params![])
            .unwrap();
    }
    #[cfg(test)]
    mod test_table {
        // use super::*;

        #[test]
        fn test_create_table() {
            assert_eq!(2 + 2, 4)
        }
    }

    pub mod get_fields {}
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
        pub fn get_user() -> User {
            User {
                Firstname: "oliver".to_string(),
                Surname: "turner".to_string(),
                YearGroup: "U6".to_string(),
                Middlename: "benjamin".to_string(),
                Preferredname: "benji".to_string(),
                Username: "turnero".to_string(),
                Status: "awol".to_string(),
                Year: 2020,
                Email: "turnero".to_string(),
                SchoolID: "soafj".to_string(),
                IsamsID: "sdanoif".to_string(),
                IsamsCode: "asoeiwrqu".to_string(),
                Title: "sir".to_string(),
                LastActive: "yesterday".to_string(),
                PupilType: "student".to_string(),
                AcademicStudy: "CS".to_string(),
                Positions: "prefect".to_string(),
                ExpoID: "8usdadfa987f9as".to_string(),
                Archived: 0,
                House: "benson".to_string(),
                Roles: "admin".to_string(),
            }
        }
    }

    pub mod schema {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        #[derive(Debug, PartialEq, PartialOrd)]
        pub struct Table {
            pub name: String,
            pub columns: Vec<(String, String)>, // name -> type
            pub foreign_keys: Vec<(String, String, String)>, // local field, foreign table, foreign field
        }
        
        pub fn get_ddl(path: String) -> impl Iterator<Item = String> {
            BufReader::new(File::open(path).unwrap())
                .lines()
                .map(Result::unwrap)
        }
    }

    pub mod db_meta {
        use rusqlite::Connection;

        pub fn connect_db(path: &str) -> Connection {
            Connection::open(&path).unwrap()
        }

        pub fn import_table(tabStr: String) -> super::schema::Table {
            let tabStr = tabStr
                .lines()
                .map(|s| s.split_whitespace().collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            let mut name: String = String::new();
            let mut columns: Vec<(String, String)> = vec![];
            let mut keys: Vec<(String, String, String)> = vec![];
            for line in tabStr {
                if &line[0] == &"FOREIGN" {
                    let x: &[_] = &[',', ';', ')'];
                    let foreign_table: Vec<&str> = line[4][..line[4].len() - 2]
                        .trim_end_matches(x)
                        .split("(")
                        .collect();
                    keys.push((
                        line[2][1..line[2].len() - 1].to_string(),
                        foreign_table[0].to_string(),
                        foreign_table[1].to_string(),
                    ));
                } else if &line[0] == &"CREATE" {
                    name = line[2].to_string();
                } else {
                    let x: &[_] = &[',', ';', ')', '('];
                    columns.push((
                        line[0]
                            .trim_start_matches(x)
                            .trim_end_matches(x)
                            .to_string(),
                        line[1..]
                            .iter()
                            .map(|y| y.trim_start_matches(x).trim_end_matches(x).to_string())
                            .collect::<Vec<String>>()
                            .join(" "),
                    ));
                }
            }
            if keys.len() == 0 {
                keys.push(("NULL".to_string(), "NULL".to_string(), "NULL".to_string()))
            }
            super::schema::Table {
                name: name,
                columns: columns,
                foreign_keys: keys,
            }
        }

        pub fn format_table(tab: super::schema::Table) -> String {
            let mut sql = String::new();
            sql.push_str("CREATE TABLE IF NOT EXISTS ");
            sql.push_str(&tab.name);
            sql.push_str(" (");
            for column in tab.columns {
                sql.push_str(&column.0);
                sql.push_str(" ");
                sql.push_str(&column.1);
                sql.push_str(", ");
            }
            if !(&tab.foreign_keys[0].0 == &"NULL".to_string()) {
                for key in tab.foreign_keys {
                    sql.push_str("FOREIGN KEY (");
                    sql.push_str(&key.0);
                    sql.push_str(") REFERENCES ");
                    sql.push_str(&key.1);
                    sql.push_str("(");
                    sql.push_str(&key.2);
                    sql.push_str("), ");
                }
            }
            let mut sql = sql[..sql.len() - 2].to_string();
            sql.push_str(")");
            sql
        }
        #[cfg(test)]
        mod test_db_meta {
            use super::*;
            #[test]
            fn test_format_table() {
                use super::super::schema::Table;
                let table = Table {
                    name: "person".to_string(),
                    columns: vec![
                        ("id".to_string(), "INTEGER PRIMARY KEY".to_string()),
                        ("name".to_string(), "TEXT NOT NULL".to_string()),
                        ("data".to_string(), "BLOB".to_string()),
                    ],
                    foreign_keys: vec![(
                        "id".to_string(),
                        "class".to_string(),
                        "class_id".to_string(),
                    )],
                };
                let person = "CREATE TABLE IF NOT EXISTS person (id INTEGER PRIMARY KEY, name TEXT NOT NULL, data BLOB, FOREIGN KEY (id) REFERENCES class(class_id))";
                assert_eq!(format_table(table), person)
            }

            #[test]
            fn test_import_table() {
                let sql = "CREATE TABLE yeargroup\n(YearID Text PRIMARY KEY,\nName Text,\nNationalCurriculumYear Text,\nCode Text,\nHeadOfYear Text,\nAssistantHeadOfYear Text,\nFOREIGN KEY (HeadOfYear) REFERENCES users(Username),\nFOREIGN KEY (AssistantHeadOfYear) REFERENCES users(Username));"
                    .to_string();
                let result = super::super::schema::Table {
                    name: "yeargroup".to_string(),
                    columns: vec![
                        ("YearID".to_string(), "Text PRIMARY KEY".to_string()),
                        ("Name".to_string(), "Text".to_string()),
                        ("NationalCurriculumYear".to_string(), "Text".to_string()),
                        ("Code".to_string(), "Text".to_string()),
                        ("HeadOfYear".to_string(), "Text".to_string()),
                        ("AssistantHeadOfYear".to_string(), "Text".to_string()),
                    ],
                    foreign_keys: vec![
                        (
                            "HeadOfYear".to_string(),
                            "users".to_string(),
                            "Username".to_string(),
                        ),
                        (
                            "AssistantHeadOfYear".to_string(),
                            "users".to_string(),
                            "Username".to_string(),
                        ),
                    ],
                };
                assert_eq!(result, import_table(sql))
            }
        }
    }
}
