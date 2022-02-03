use postgres::{Client, Error, NoTls};
use std::collections::HashMap;

fn main() {
    if let Err(err) = create_table() {
        println!("创建数据表错误：{}", err);
    }
    if let Err(err) = insert_data() {
        println!("插入数据错误：{}", err);
    }
    if let Err(err) = query_data() {
        println!("查询数据错误：{}", err);
    }
}

fn create_table() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgres@localhost:5432/library",
        NoTls,
    )?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS author (
            id      SERIAL PRIMARY KEY,
            name    VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        )
    ",
    )?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS book(
            id          SERIAL PRIMARY KEY,
            title       VARCHAR NOT NULL,
            author_id   INTEGER NOT NULL REFERENCES author
        )
    ",
    )?;

    Ok(())
}

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn insert_data() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgres@localhost:5432/library",
        NoTls,
    )?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    Ok(())
}

fn query_data() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgres@localhost:5432/library",
        NoTls,
    )?;

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
