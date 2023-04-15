pub mod dataBased{
    
    use std::{collections::HashMap, hash::Hash};

    use chrono::Utc;

    #[derive(Debug)]
    pub struct Workspace{
        name: String,
        author: String,
        metadata: String,
        database:HashMap<String,Db>,
        logger:bool,
    }

    #[derive(Debug)]    
    pub struct Db{
        name:String,
        table: HashMap<String,Table>
    }
    
    #[derive(Debug)]
    pub struct Table{
    
        headers: Vec<String>,
        model:Vec<String>,
        cells: Vec<Vec<String>>,
        relations: Vec<Relation>
    
    }

    #[derive(Debug)]
    pub struct Relation{
        table_name:String,
        col_name:String
    }

    impl Workspace{

        pub fn getName(&self) -> &String{

            return &self.name;

        }

        pub fn getAuthor(&self) -> &String{

            return &self.author;

        }

        pub fn getMetadata(&self) -> &String{

            return &self.metadata;

        }

        pub fn getDB(&self) -> &HashMap<String, Db>{

            return &self.database;

        }

        fn getDBNames(&self) -> String{

            let g = &self.database;

            let mut x = String::from("[");

            for (k,v) in g{

                x.push_str(&k);
                x.push_str(", ");

            }
            x = x[0..x.len()-2].to_owned();
            x.push_str("]");

            return x;

        }

        pub fn print(&self){

            println!();
            println!("Name => {:?}", self.getName());
            println!("Author => {:?}", self.getAuthor());
            println!("Metadata => {:?}", self.getMetadata());
            println!("Databases => {:?}", self.getDBNames());

        }

        pub fn addDB(&mut self,x:String){

            let y = Db{
                name:x.clone(),
                table:HashMap::new()
            };

            let _ = &self.database.insert(x,y);

        }

    }
    
    pub fn create_workspace(x:String, y:String) -> Workspace{

        let mut meta = String::from("");

        meta.push_str("Date Created : ");
        meta.push_str(&Utc::now().to_string());

        let x = Workspace{

            name: x,
            author: y,
            metadata: meta,
            database: HashMap::new(),
            logger:false

        };

        return x;

    }

}

fn main() {

    let mut g = dataBased::create_workspace("HolyDB".to_owned(), "Nabeel Mirza".to_owned());

    g.addDB("Students".to_owned());
    g.addDB("Employees".to_owned());
    g.print();
 
}
