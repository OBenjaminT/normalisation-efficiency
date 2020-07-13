pub mod table {
    #![allow(dead_code, non_snake_case)]
    use rusqlite::{params, Connection};

    pub fn create_table(conn: &Connection, set: schema::Table) {
        conn.execute("DROP TABLE IF EXISTS person;", params![])
            .unwrap();
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
        pub fn get_person() -> User {
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
        #[derive(Debug, PartialEq, PartialOrd)]
        pub struct Table {
            pub name: String,
            pub columns: Vec<(String, String)>, // name -> type
            pub foreign_keys: Vec<(String, String, String)>, // local field, foreign table, foreign field
        }

        pub mod no_sql_startingpoint {
            fn get_yeargroup_table() -> String {
                let sql = "CREATE TABLE yeargroup \n(YearID Text PRIMARY KEY,\nName Text,\nNationalCurriculumYear Text,\nCode Text,\nHeadOfYear Text,\nAssistantHeadOfYear Text,\nFOREIGN KEY (HeadOfYear) REFERENCES users(Username),\nFOREIGN KEY (AssistantHeadOfYear) REFERENCES users(Username));".to_string();
                sql
            }

            fn get_year_table() -> String {
                let sql = "CREATE TABLE year \n(yearID Integer PRIMARY KEY,\nStartYear Text,\nEndYear Text,\nStartDate Text,\nEndDate Text,\nCurrentYear Integer,\nTermsTermID Integer,\nTermsName Text,\nTermsStartDate Text,\nTermsEndDate Text);".to_string();
                sql
            }

            fn get_users_table() -> String {
                let sql = "CREATE TABLE users \n(Firstname Text,\nSurname Text,\nYearGroup Text,\nMiddlename Text,\nPreferredname Text,\nUsername Text PRIMARY KEY,\nStatus Text,\nYear Integer,\nEmail Text,\nSchoolID Text,\nIsamsID Text,\nIsamsCode Text,\nTitle Text,\nLastActive Text,\nPupilType Text,\nAcademicStudy Text,\nPositions Text,\nExpoID Text,\nArchived Integer DEFAULT 0,\nHouse Text,\nRoles Text,\nFOREIGN KEY (YearGroup) REFERENCES yeargroup(YearID),\nFOREIGN KEY (House) REFERENCES house(HouseID),\nFOREIGN KEY (Roles) REFERENCES role(Name),\nFOREIGN KEY (Username) REFERENCES tutorGroup(Students),\nFOREIGN KEY (Username) REFERENCES Sets(Students),\nFOREIGN KEY (Username) REFERENCES prep(DueForUser),\nFOREIGN KEY (Username) REFERENCES lesson(Students));".to_string();
                sql
            }

            fn get_tutorGroup_table() -> String {
                let sql = "CREATE TABLE tutorGroup \n(Name Text PRIMARY KEY,\nTutors Text,\nStudents Text,\nAcademicYear Text,\nYear Text,\nArchived Integer DEFAULT 0,\nHouse Text,\nFOREIGN KEY (Tutors) REFERENCES users(Username),\nFOREIGN KEY (AcademicYear) REFERENCES year(yearID),\nFOREIGN KEY (House) REFERENCES house(HouseID));".to_string();
                sql
            }

            fn get_subject_table() -> String {
                let sql = "CREATE TABLE subject \n(SubjectID Text PRIMARY KEY,\nDepartment Text,\nName Text,\nCode Text,\nReportName Text,\nFOREIGN KEY (Department) REFERENCES department(DepartmentID));".to_string();
                sql
            }

            fn get_Sets_table() -> String {
                let sql = "CREATE TABLE Sets \n(SetsID Text PRIMARY KEY,\nSubject Text,\nName Text,\nYear Text,\nArchived Integer DEFAULT 0,\nTeacher Text,\nStudents Text,\nYearGroup Text,\nFOREIGN KEY (Subject) REFERENCES subject(SubjectID),\nFOREIGN KEY (Year) REFERENCES year(yearID),\nFOREIGN KEY (Teacher) REFERENCES users(Username),\nFOREIGN KEY (YearGroup) REFERENCES yeargroup(YearID));".to_string();
                sql
            }

            fn get_room_table() -> String {
                let sql = "CREATE TABLE room \n(Building Text,\nCode Text PRIMARY KEY,\nName Text,\nInitials Text,\nFOREIGN KEY (Building) REFERENCES building(BuildingID));".to_string();
                sql
            }

            fn get_role_table() -> String {
                let sql = "CREATE TABLE role \n(Name Text PRIMARY KEY,\nAuth Text,\nType Text);"
                    .to_string();
                sql
            }

            fn get_prep_table() -> String {
                let sql = "CREATE TABLE prep \n(Sets Text,\nDueLesson Text,\nTitle Text PRIMARY KEY,\nDescription Text,\nCreatedAt Text,\nDueAt Text,\nDueForCompleted Integer DEFAULT 0,\nDueForUpdatedAt Text,\nDueFor Text,\nPersonal Integer,\nLastEditedAt Text,\nAssignedDate Text,\nArchived Integer DEFAULT 0,\nDueForUser Text,\nAssignedBy Text,\nFiles Text,\nFOREIGN KEY (Sets) REFERENCES Sets(SetsID),\nFOREIGN KEY (DueLesson) REFERENCES lesson(lessonID),\nFOREIGN KEY (AssignedBy) REFERENCES users(Username));".to_string();
                sql
            }

            fn get_period_table() -> String {
                let sql = "CREATE TABLE period \n(PeriodID Text PRIMARY KEY,\nDay Text,\nStartTime Text,\nEndTime Text,\nLongName Text,\nShortName Text,\nWeek Text);".to_string();
                sql
            }

            fn get_notification_table() -> String {
                let sql = "CREATE TABLE notification \n(Sender Text,\nRecipientsUser Text,\nTitle Text PRIMARY KEY,\nMessage Text,\nRecipientsRead Integer,\nRecipientsReadAt Text,\nSentAt Text,\nMetaPage Text,\nMetaParam Text,\nFOREIGN KEY (Sender) REFERENCES users(Username),\nFOREIGN KEY (RecipientsUser) REFERENCES users(Username));".to_string();
                sql
            }

            fn get_module_table() -> String {
                let sql = "CREATE TABLE module \n(CreatedBy Text,\nName Text PRIMARY KEY,\nIcon Text,\nLink Text,\nExternalMod Integer,\nColour Text,\nShowFor Text,\nPreviewTitle Text,\nPreviewSubtitle Text,\nPreviewLink Text,\nStaff Integer,\nNew Integer DEFAULT 0,\nFOREIGN KEY (CreatedBy) REFERENCES users(Username));".to_string();
                sql
            }

            fn get_lesson_table() -> String {
                let sql = "CREATE TABLE lesson \n(lessonID Integer PRIMARY KEY,\nSets Text,\nArchived Integer DEFAULT 0,\nSubject Text,\nStart Text,\nRoom Text,\nEndTime Text,\nPeriod Text,\nTeacher Text,\nStudents Text,\nFOREIGN KEY (Sets) REFERENCES Sets(SetsID),\nFOREIGN KEY (Subject) REFERENCES subject(SubjectID),\nFOREIGN KEY (Room) REFERENCES room(Code),\nFOREIGN KEY (Period) REFERENCES period(PeriodID),\nFOREIGN KEY (Teacher) REFERENCES users(Username),\nFOREIGN KEY (Archived) REFERENCES lesson(Sets));".to_string();
                sql
            }

            fn get_house_table() -> String {
                let sql = "CREATE TABLE house \n(Name Text PRIMARY KEY,\nCode Text,\nType Text,\nHousemaster Text,\nAssistantHousemaster Text,\nLogo Text,\nHouseID Text,\nFOREIGN KEY (Housemaster) REFERENCES users(Username),\nFOREIGN KEY (AssistantHousemaster) REFERENCES users(Username));".to_string();
                sql
            }

            fn get_files_table() -> String {
                let sql = "CREATE TABLE files \n(fileID Integer PRIMARY KEY,\nUploadedBy Text,\nS3Key Text,\nContentType Text,\nFileName Text,\nTitle Text,\nUploadedAt Text,\nFOREIGN KEY (fileID) REFERENCES prep(Files),\nFOREIGN KEY (UploadedBy) REFERENCES users(Username));".to_string();
                sql
            }

            fn get_department_table() -> String {
                let sql = "CREATE TABLE department \n(DepartmentID Text PRIMARY KEY,\nName Text,\nHeadOfDepartment Text,\nFOREIGN KEY (HeadOfDepartment) REFERENCES users(Username));".to_string();
                sql
            }

            fn get_building_table() -> String {
                let sql = "CREATE TABLE building \n(BuildingID Text PRIMARY KEY,\nName Text,\nInitials Text,\nDescription Text);".to_string();
                sql
            }

            pub fn get_no_sql_startingpoing() -> Vec<String> {
                let no_sql: Vec<String> = vec![
                    super::no_sql_startingpoint::get_yeargroup_table(),
                    super::no_sql_startingpoint::get_year_table(),
                    super::no_sql_startingpoint::get_users_table(),
                    super::no_sql_startingpoint::get_tutorGroup_table(),
                    super::no_sql_startingpoint::get_subject_table(),
                    super::no_sql_startingpoint::get_Sets_table(),
                    super::no_sql_startingpoint::get_room_table(),
                    super::no_sql_startingpoint::get_role_table(),
                    super::no_sql_startingpoint::get_prep_table(),
                    super::no_sql_startingpoint::get_period_table(),
                    super::no_sql_startingpoint::get_notification_table(),
                    super::no_sql_startingpoint::get_module_table(),
                    super::no_sql_startingpoint::get_lesson_table(),
                    super::no_sql_startingpoint::get_house_table(),
                    super::no_sql_startingpoint::get_files_table(),
                    super::no_sql_startingpoint::get_department_table(),
                    super::no_sql_startingpoint::get_building_table(),
                ];
                no_sql
            }
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
