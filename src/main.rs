use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;


fn main(){
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

    // First of all we load the file db.txt into our map (struct) on memory.
    let mut todo = Todo::new_vs2().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error ocurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error ocurred: {}", why),
            },
        }
    }
}


struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {

    // It returns a Result that is Todo or IO::Error
    fn new() -> Result<Todo, std::io::Error> {

        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines() // Iterates each line of our file content.
            // on each element of the iterator
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            // we transform it into a tuple
            .map(|v| (v[0], v[1]))
            // we convert the two elements of the tuple
            // into a String and a boolean
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            // Finally we convert it into our HashMap
            .collect();
        Ok(Todo {map})
    }

    // Another implementation of the new method with for loop
    fn new_vs2() -> Result<Todo, std::io::Error> {
        // open db file
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        // read its content into a new string
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        // allocate an empty HashMap
        let mut map = HashMap::new();

        // loop over each lines of the file
        for entries in content.lines() {
            // split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No value");
            // insert them into HashMap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        // Return Ok
        Ok(Todo { map })
    }

    // new method version for JSON 
    fn new_json() -> Result<Todo, std::io::Error> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // serialize json as HashMap
        // and tries to convert the JSON into a
        // compatible HashMap (String, boolean) style...
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            // If the file is empty we create a new HashMap.
            Err(e) if e.is_eof() => Ok (Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error ocurredf: {}", e),
        } 
    }

    // save method version for JSON
    fn save_json(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(()) // the return value

    }


    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    // Save function takes the ownership of self (map)
    // so the compiler stops us if we accidentally try to update
    // the map after calling save.
    // We enforce save as the last method to be use.
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record); // add record (line) to the content string
        }
        std::fs::write("db.txt", content) // without ; because is the returning value
    }

    // Returns an empty Option
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

}

