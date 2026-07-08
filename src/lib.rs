use rusqlite::{Connection, Result};
use console::style;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub struct Item {
    pub id : i32,
    pub title : String,
    pub date_added : String,
    pub is_done : u8
}

impl Item {
    pub fn print_data(self : &Self) {
        let status = match self.is_done {
            1 => style("Done").green(),
            0 => style("Pending").red(),
            _ => unreachable!("Invalid is_done value")
        };
        println!("{:>4} | {:<44}  {:<8}  {}", 
            style(self.id).cyan(), 
            style(truncate_at(&self.title, 44)), 
            status, 
            style(&self.date_added).dim()
        );
    }
}

pub fn get_connection() -> Result<Connection> {
    let dir = String::from("C:\\Users\\Personal\\Documents");
    let db_path = format!("{dir}\\todo.sqlite");
    let conn = Connection::open(db_path)?;
    seed_db(&conn)?;

    Ok(conn)
}

pub fn add(conn : &Connection, title : String, ) -> Result<()> {
    conn.execute("INSERT INTO items (title) VALUES(?);", 
    [title])?;
    Ok(())
}

pub fn toggle(conn : &Connection, id : i32) -> Result<()> {
    conn.execute("UPDATE items SET is_done=1 - is_done WHERE id=?", 
    [id])?;
    Ok(()) 
}

pub fn del(conn : &Connection, id : i32) -> Result<()> {
    conn.execute("DELETE FROM items WHERE id=?", 
    [id])?;
    Ok(()) 
}

pub fn list(conn : &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM items;")?;
    let iter = stmt.query_map([], |row| {
        Ok(crate::Item {
            id : row.get(0)?,
            title : row.get(1)?,
            date_added : row.get(2)?,
            is_done : row.get(3)?,
        })
    })?;

    for item in iter {
        item?.print_data();
    }

    Ok(())
}

pub fn reset(conn : &Connection) -> Result<()> {
    conn.execute("DELETE FROM items;", ())?;
    Ok(())
}

pub fn truncate_at(input: &str, max: i32) -> String {
    let max_len: usize = max as usize;
    if input.len() > max_len {
        let truncated = &input[..(max_len - 3)];
        return format!("{}...", truncated);
    };

    input.to_string()
}

pub fn seed_db(conn : &Connection) -> Result<()> {
    conn.execute("
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            date_added TEXT NOT NULL DEFAULT current_timestamp,
            is_done BOOLEAN NOT NULL DEFAULT 0
        );
    ", [])?;
    Ok(())
}

pub fn get_id() -> i32 {
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line!");
    let id = id.trim().parse().expect("Not an integer!");
    id
}

pub fn print_options() {
    println!("<1> Add Task");
    println!("<2> Delete Task");
    println!("<3> List Tasks");
    println!("<4> Toggle Task");
    println!("<5> Reset Tasks");
    println!("<6> Exit");
    print!(">> ");
    io::stdout().flush().expect("Failed to flush.");
}