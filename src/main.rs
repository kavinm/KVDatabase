use std::{collections::HashMap};

 fn main() {
    //iterator skip first n elements, in this case it will skip kvstore
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key is not present");
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}' ", key, value);




    let kv_path = std::path::Path::new("kv.db");

    if kv_path.exists(){
        let mut database = Database::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    match database.flush(){
        Ok(()) => println!("Succesfully added key value pair to database"),
        Err(err) => println!("Error: {}", err)
    }
}

    

    


  
} 


//Database struct fields
struct Database{
    map: HashMap<String, String>,
    flush: bool
}

//methods for Database
impl Database{

    //function called new that returns a Database, new is not a keyword in rust
    fn new() -> Result<Database, std::io::Error>{

        let mut map = HashMap::new();

        // read the kv.db file
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines(){
            let (key, value ) = line.split_once('\t').expect("Corrupt database");

            //if you insert a key that already exists, it returns the existing key/value
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database{map, flush: false})
    }

    //method for tinserting into database
    fn insert(&mut self, key: String, value: String){
        self.map.insert(key, value);
    }

    //method for writing to database
    fn flush(mut self) -> std::io::Result<()>{
        self.flush = true;
       do_flush(&self)
    }

}

//called when a Database value goes out of scope and gets "dropped"
impl Drop for Database{
    fn drop(&mut self){
        if !self.flush{
            let _ = do_flush(self);
        }
        
    }
}

fn do_flush(database: &Database) -> std::io::Result<()>{
    println!("do flush is called");
    let mut contents = String:: new();
    for (key, value) in &database.map{


        //less efficient code because not stored in a :

        //let kvpair: String = format!("{}\t{}\n", key, value);
        //contents.push_str(&kvpair);

        contents.push_str(&key);
        contents.push('\t');
        contents.push_str(&value);
        contents.push('\n');

    }

    std::fs::write("kv.db", contents)
    
}


