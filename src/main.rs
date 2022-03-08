use std::collections::HashMap;

fn main() {
    // get arguments from command line ($ kvstore param1 param2 ...)
    // cargo run -- hello world
    let mut arguments = std::env::args().skip(1);
    // get Objects Option<String>
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is'{}'", key, value);
    let contents = format!("{}\t{}", key, value);
    // Write to file
    let write_result = std::fs::write("kv.db", contents);
    //  Result<(), std::io::Error>
    match write_result {
        Ok(_) => println!("Successfully wrote to the file"),
        Err(e) => println!("There was an error writing to the file: {:?}", e),
    }
    let database = Database::new().expect("Database::new failed");
}

struct Database {
    map: std::collections::HashMap<String, String>,
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
        Ok(Database { map: map })
    }
}
