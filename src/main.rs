use rusqlite::{params, Result};
mod back_end;
use back_end::table as t;
use back_end::table::db_meta;
use back_end::table::schema;
use back_end::table::get_tables;

fn main() -> Result<()> {
    let conn = db_meta::connect_db("test.db");
    for table_sql in schema::no_sql_startingpoint::get_no_sql_startingpoing().iter() {
        t::create_table(&conn, db_meta::import_table(table_sql.to_string()));
    }
    let me = get_tables::get_person();

    conn.execute(
        "INSERT INTO users (Firstname, Surname, YearGroup, Middlename, Preferredname, Username, Status, Year, Email, SchoolID, IsamsID, IsamsCode, Title, LastActive, PupilType, AcademicStudy, Positions, ExpoID, Archived, House, Roles) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
        params![
            me.Firstname,
            me.Surname,
            me.YearGroup,
            me.Middlename,
            me.Preferredname,
            me.Username,
            me.Status,
            me.Year,
            me.Email,
            me.SchoolID,
            me.IsamsID,
            me.IsamsCode,
            me.Title,
            me.LastActive,
            me.PupilType,
            me.AcademicStudy,
            me.Positions,
            me.ExpoID,
            me.Archived,
            me.House,
            me.Roles
        ],
    )?;

    let mut stmt = conn.prepare("SELECT * FROM users")?;
    let user_iter = stmt.query_map(params![], |row| {
        Ok(get_tables::User {
            Firstname: row.get(0)?,
            Surname: row.get(1)?,
            YearGroup: row.get(2)?,
            Middlename: row.get(3)?,
            Preferredname: row.get(4)?,
            Username: row.get(5)?,
            Status: row.get(6)?,
            Year: row.get(7)?,
            Email: row.get(8)?,
            SchoolID: row.get(9)?,
            IsamsID: row.get(10)?,
            IsamsCode: row.get(11)?,
            Title: row.get(12)?,
            LastActive: row.get(13)?,
            PupilType: row.get(14)?,
            AcademicStudy:row.get(15)?, 
            Positions: row.get(16)?,
            ExpoID: row.get(17)?,
            Archived: row.get(18)?,
            House: row.get(19)?,
            Roles: row.get(20)?,
        })
    })?;

    for user in user_iter {
        println!("Found person {:?}", user.unwrap());
    }
    Ok(())
}
