use std::collections::HashMap;

fn main() {
    //iterator skip first n elements, in this case it will skip kvstore
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}' ", key, value);

    let contents = format!("{}\t{}\n", key, value);

    //returns a Result which is an enum, can be () which is unit, or Error

    let KVpath = std::path::Path::new("kv.db");

    if KVpath.exists(){

    }

    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() crashed");

  
} 


//create the struct with fields
struct Database{
    map: HashMap<String, String>,
}

//this is where you have the methods
impl Database{

    //function called new that returns a Database, new is not a keyword in rust
    fn new() -> Result<Database, std::io::Error>{
        // read the kv.db file

        // let contents = match std::fs::read_to_string("kv.db"){
        //     Ok(c) => c,

        //     Err(error) =>{
        //         return Err(error);
        //     }
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines(){
            let (key, value ) = line.split_once('\t').expect("Corrupt database");

            //if you insert a key that already exists, it returns the existing key/value
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse the string
        //populate our map  
        Ok(Database{
            map: map 
        })
    }

}


