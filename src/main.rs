use std::io::{self, Read, Write};
use rusqlite::Result;
use todo_list::{add, del, get_connection, list, print_options, reset, toggle, get_id};

fn main() -> Result<()> {
    let conn = get_connection()?;
    let mut choice = String::new();
    
    loop {
        print_options();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim() {
            "1" => {
                let mut title = String::new();
                print!("Title: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line.");
                add(&conn, title)?;
            },
            "2" => {
                print!("Task Id: ");
                let id = get_id();
                del(&conn, id)?;
            },
            "3" => {
                list(&conn)?;
            },
            "4" => {
                print!("Task Id: ");
                let id = get_id();
                toggle(&conn, id)?;
            },
            "5" => {
                reset(&conn)?;
            },
            "6" => {
                break;
            },
            _ => println!("Invalid Choice!")
        }
        choice.clear();
    }

    Ok(())
}
