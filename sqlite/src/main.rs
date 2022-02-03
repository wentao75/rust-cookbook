use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;
    if let Err(err) = create_table(&mut conn) {
        println!("创建数据表错误：{}", err);
    }

    if let Err(err) = insert_data(&mut conn) {
        println!("插入数据错误：{}", err);
    }

    Ok(())
}

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

fn create_table(conn: &mut Connection) -> Result<()> {
    // let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (id integer primary key, name text not null unique)",
        NO_PARAMS,
    )?;
    conn.execute("create table if not exists cats (id integer primary key, name text not null, color_id integer not null references cat_colors(id))", NO_PARAMS,)?;
    Ok(())
}

fn insert_data(conn: &mut Connection) -> Result<()> {
    // let conn = Connection::open("cats.db")?;

    conn.execute("delete from cat_colors", NO_PARAMS)?;
    conn.execute("delete from cats", NO_PARAMS)?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values(?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c INNER JOIN cat_colors cc ON cc.id=c.color_id;",
    )?;

    let cats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values(?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values(?1)", &[&"blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values(?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values(?1)", &[&"blue"])?;
    tx.execute("insert into cat_colors (name) values(?1)", &[&"lavender"])?;

    tx.commit()
}
