#![allow(non_snake_case)]
use core::slice;
use std::collections::HashMap;
fn main(){
    let mut args = std::env::args().skip(1);
    let guide = args.next().expect("you are very cringe");
    let key = args.next().expect("key not found");
    let mut value = args.next().expect("insert _ for value if you are trying to refrense a value");
    let mut database = Database::new().expect("creating db failed ");
    
    if guide.eq_ignore_ascii_case(&"reference") {
        value = database.pullvalue(&key).unwrap();
        println!("the key is {} the value is {}", key, value);  
    }
    
    
    if guide.eq_ignore_ascii_case(&"input"){
    println!("the key is {} the value is {}", key, value); 
    let _contents = format!("{}\t{}\n", key , value);
    // value.clone() becuse we are dumb and memory is hard.
    database.insert(key.to_uppercase(),value.clone());
    database.insert(key,value);
    database.flush().unwrap()
    }
 

}

struct Database {
    map: std::collections::HashMap<String, String>,
}

impl Database{
    fn new() -> Result<Database, std::io::Error> {
        // read kv.db file pase the string populate the map 
    //    let contents = match std::fs::read_to_string( "kv.db"){
    //        Ok(c) => c,
    //        Err(error) => {
    //            return Err(error);
    //        }                                             
    //    } == let contents = std::fs::read_to_string("kv.db")?;
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2,'\t');
            //&str is a string slice or a refreense to an owned line(pointer and length) of contents 
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
            // .to_owned transfers key and value in to copeys that are String not &str 
            // this is to keep key and value in scope 
            map.insert(key.to_owned(),value.to_owned()); 
        }
        // Ok(Database{ map: map })
        Ok(Database{map})
    }
    //&mut self allowes self to be borowed with out ownership being moved to self
    fn insert(&mut self, key: String, value: String){
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key , value) in self.map{
            //let kvpair =format!("{}\t{}\n", pairs.0,pairs.1 ); > less efficent but less lines
            contents.push_str(&key);
            contents.push('\t');
            contents.push_str(&value);
            contents.push('\n');
        }  
        std::fs::write("kv.db",contents)
    }

    fn pullvalue(&self, key: &String) -> Result<String, &'static str> {
        if let Some(returned) = self.map.get(key) {
            return Ok(returned.to_owned());
        }
        Err("no value assosiated with this key")
    }

}