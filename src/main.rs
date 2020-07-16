use rusqlite::{params, Result};
mod back_end;
use back_end::table as t;
use back_end::table::get_tables;
use rusqlite::Connection;

fn main() {
    actal_main().unwrap();
}

fn actal_main() -> Result<()> {
    let conn = Connection::open("test.db").unwrap();
    for sql in t::get_ddl("src/sql/noSQL/drop.txt".to_string()) {
        conn.execute(&sql[..], params![])?;
    }
    for sql in t::get_ddl("src/sql/noSQL/create.txt".to_string()) {
        conn.execute(&sql[..], params![])?;
    }

    let rollee = t::fill_vec_str(3);
    conn.execute(
        "INSERT INTO role VALUES (?1, ?2, ?3);",
        params![rollee[0], rollee[1], rollee[2]],
    )?;

    let mut stmt = conn.prepare("SELECT * FROM role")?;
    let role_iter = stmt.query_map(params![], |row| {
        Ok(get_tables::Role {
            name: row.get(0)?,
            auth: row.get(1)?,
            typeS: row.get(2)?,
        })
    })?;

    for role in role_iter {
        println!("Found person {:?}", role.unwrap());
    }
    Ok(())
}
