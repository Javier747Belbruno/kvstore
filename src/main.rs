use std::collections::HashMap;

fn main() {
    // get arguments from command line ($ kvstore param1 param2 ...)
    // cargo run -- hello world
    let mut arguments = std::env::args().skip(1);
    // get Objects Option<String>
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is'{}'", key, value);
    let mut database = Database::new().expect("Database::new failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    match database.flush() {
        Ok(_) => println!("Flushed"),
        Err(e) => println!("Error: {}", e),
    }
}

struct Database {
    map: std::collections::HashMap<String, String>,
    flush: bool,
}
//Here we are implementing the methods
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut parts = line.split("\t");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map: map,
            flush: false,
        })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            println!("Dropping");
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    println!("Do flush called");
    let mut contents = String::new();
    for (key, value) in &database.map {
        let kvpair = format!("{}\t{}\n", key, value);
        contents.push_str(&kvpair);
    }
    std::fs::write("kv.db", contents)
}
