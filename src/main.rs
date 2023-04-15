pub mod dataBased_Logger{

}
pub mod dataBased{
    
    use std::{collections::HashMap, hash::Hash, fmt::Error};

    use chrono::Utc;
    use colored::Colorize;

    #[derive(Debug)]
    struct Logger{
        code:Vec<i32>,
        positives:bool
    }

    impl Logger{

        pub fn reset(&mut self){
            self.code = Vec::new();
        }

        pub fn reveal(&mut self)->usize{
            return self.code.clone().len();
        }

        pub fn update(&mut self, x:i32, y:String){
            &self.code.push(x);
            self.throw(y);
        }

        fn throw(&mut self, y:String){

            let x = self.getLen();
            let error_code = self.code[x-1];

            if  !self.positives && error_code > 0 {
                return;
            }

            let mut e_string = error_code.to_string();
            e_string.push_str(" ");

            println!();
            match error_code{

                -1 =>{
                    println!("{}{}| {} {} {}", " Error Code ".on_red() ,e_string.on_red() ,"Database named", &y.cyan(), "does not exist in the current workplace!");
                }
                -3 =>{
                    println!("{}{}| {} {} {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Database named", &y.cyan(), "already exists in workspace!");
                }
                _ =>{
                    println!("{}{}| {}", " Undocumented Code ".on_truecolor(0,0,255), e_string.to_string().on_truecolor(0,0,255), "No definition found for this error!");
                }


            }

        }

        fn getLen(&mut self)-> usize{
            return self.code.len();
        }

        pub fn toggle(&mut self){
            self.positives = !self.positives;
        }
    }

    #[derive(Debug)]
    pub struct Workspace{
        name: String,
        author: String,
        metadata: String,
        database:HashMap<String,Db>,
        logger: Logger
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

        pub fn throwUC(&mut self){
            //Force throws undocumented error code exception!
            self.logger.update(-99999,"".to_owned());

        }

        pub fn toggleLogger(&mut self){
            self.logger.toggle();
        }

        fn getDBNames(&self) -> String{

            let g = &self.database;

            let mut x = String::from("[");

            for (k,v) in g{

                x.push_str(v.getName());
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

            if self.database.contains_key(&x){
                self.logger.update(-3, x);
                return;
            }

            let y = Db{
                name:x.clone(),
                table:HashMap::new(),
            };

            let _ = &self.database.insert(x,y);

        }

        pub fn fetchDB(&self)-> &HashMap<String,Db>{

            return &self.database;

        }

        pub fn fetchDB_name(&mut self, x:String) -> bool{

            if self.database.contains_key(&x) {
                return true;
            }else{
                self.logger.update(-1,x);
                return false;
            }

        }

    }
    

    #[derive(Debug)]    
    pub struct Db{
        name:String,
        table: HashMap<String,Table>,
    }

    impl Db{

        fn getName(&self)-> &String{
            return &self.name;
        }

        fn getTables(&self)-> &HashMap<String,Table>{
            return &self.table;
        }

        fn createTable(&mut self, x:String, y:Vec<String>, z:Vec<String>){
            
            let mut v= Table{
                name:x.clone(),
                headers:y,
                model:z,
                cells : Vec::new(),
                relations:Vec::new()
            };

            let _ = self.table.insert(x, v);

        }

    }
    
    #[derive(Debug)]
    pub struct Table{
        
        name:String,
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
    pub fn create_workspace(x:String, y:String) -> Workspace{

        let mut meta = String::from("");

        meta.push_str("Date Created : ");
        meta.push_str(&Utc::now().to_string());

        let x = Workspace{

            name: x,
            author: y,
            metadata: meta,
            database: HashMap::new(),
            logger: init_Logger()

        };

        return x;

    }

    fn init_Logger()->Logger{
        let mut L = Logger{
            code:Vec::new(),
            positives:false
        };

        return L;
    }

}

fn main() {

    let mut g = dataBased::create_workspace("HolyDB".to_owned(), "Nabeel Mirza".to_owned());

    g.addDB("Students".to_owned());
    g.addDB("Employees".to_owned());
    g.addDB("Employees".to_owned());
    g.addDB("Najam".to_owned());

    g.fetchDB_name("STD".to_owned());
    g.fetchDB_name("FOX".to_owned());
    g.fetchDB_name("AMPILOYEE".to_owned());

    g.addDB("Employees".to_owned());
    g.throwUC();
    

    g.print();

 
}
