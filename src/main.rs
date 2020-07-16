use rusqlite::{params, Result};
mod back_end;
use back_end::table as t;
use back_end::table::get_tables;
use rusqlite::Connection;

fn main() {
    actal_main();
}

fn actal_main () -> Result<()> {
    let conn = Connection::open("test.db").unwrap();
    for sql in t::get_ddl("src/sql/noSQL/drop.txt".to_string()) {
        conn.execute(&sql[..], params![])?;
    }
    for sql in t::get_ddl("src/sql/noSQL/create.txt".to_string()) {
        conn.execute(&sql[..], params![])?;
    }
    /* let me = get_tables::get_user(); */

    let rollee = t::fill_vec_str(3);
    conn.execute(
        "INSERT INTO role VALUES (?1, ?2, ?3);",
        params![rollee[0], rollee[1], rollee[2]],
    )?;

    /*     conn.execute(
        "INSERT INTO users VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
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
    )?; */

    let mut stmt = conn.prepare("SELECT * FROM role")?;
    let role_iter = stmt.query_map(params![], |row| {
        Ok(get_tables::Role {
            name: row.get(0)?,
            auth: row.get(1)?,
            typeS: row.get(2)?,
        })
        /* Ok(get_tables::User {
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
            AcademicStudy: row.get(15)?,
            Positions: row.get(16)?,
            ExpoID: row.get(17)?,
            Archived: row.get(18)?,
            House: row.get(19)?,
            Roles: row.get(20)?,
        }) */
    })?;

    for role in role_iter {
        println!("Found person {:?}", role.unwrap());
    }
    Ok(())
}