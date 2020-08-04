use rusqlite::{params, Result, Connection};
mod back_end;
use back_end::table as t;
use back_end::table::get_tables;

fn main() {
    actual_main().unwrap();
}


#[allow(dead_code)]
fn actual_main() -> Result<()> {
    let conn = Connection::open("test.db").unwrap();
    let schema = "calendar1NF";
    for sql in t::get_ddl(format!("src/sql/{}/drop.sql", schema).to_string()) {
        conn.execute(&sql[..], params![])?;
    }
    for sql in t::get_ddl(format!("src/sql/{}/create.sql", schema).to_string()) {
        conn.execute(&sql[..], params![])?;
    }

    t::insert_role(&conn, 3);

    let mut stmt = conn.prepare("SELECT * FROM role")?;
    let role_iter = stmt.query_map(params![], |row| {
        Ok(get_tables::Role {
            name: row.get(0)?,
            auth: row.get(1)?,
            typeS: row.get(2)?,
        })
    })?;

    for role in role_iter {
        println!("Found {:?}", role.unwrap());
    }
    Ok(())
}
