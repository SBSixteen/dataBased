use dataBased::generateSession;
use std::fmt::Debug;


pub mod dataBased{
    
    use std::{collections::HashMap, hash::Hash, fmt::Error, io::{self, stdout, Write}};

    use chrono::Utc;
    use colored::Colorize;
    use rand::Rng;

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
                -2 =>{
                    println!("{}{}| {} {} {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Database named", &y.cyan(), "already exists in workspace!");
                }
                -3 =>{
                    println!("{}{}| {} {} {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Table named", &y.cyan(), "already exists in this database!");
                }
                -4 =>{
                    println!("{}{}| {} {} {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Cannot create database ", &y.cyan(), "as no workspace is set in session!");
                }
                -1000  =>{
                    println!("{}{}| {} {} {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Command", &y.cyan(), "does not exist!");
                }
                -1001  =>{
                    println!("{}{}| {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"No Command typed.");
                }
                -1002  =>{
                    println!("{}{}| {} {} {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Workspace", &y.cyan(), "already exists in current session!");
                }
                -1003 =>{
                    println!("{}{}| {} {} {}", " Warning ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Workspace", &y.cyan(), "does not exist in current session!");
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

            if g.len() == 0{
                return String::from("[]")
            }

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
            for (k,v) in &self.database{

                let x = v;
                x.print();

            }

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

        pub fn fetchDB(&mut self)-> &mut HashMap<String,Db>{

            return &mut self.database;

        }

        pub fn fetchDB_name(&mut self, x:String) -> bool{

            if self.database.contains_key(&x) {
                return true;
            }else{
                self.logger.update(-1,x);
                return false;
            }

        }

        pub fn createTable(&mut self, DB:String, name:String, headers:String, model:String){

            let check_DB = self.fetchDB_name(DB.clone());

            if !check_DB{
                return;
            }

            let mut temp = self.fetchDB();
            let db = temp.get_mut(&DB).unwrap();

            let x = headers.split(",").collect::<Vec<_>>();
            let y= model.split(",").collect::<Vec<_>>();

            let mut HEAD = Vec::new(); 
            let mut MDL = Vec::new();

            //println!("{:?}",x);
            //println!("{:?}",y);

            for i in 0..x.len(){
                
                HEAD.push(x[i].to_owned());
                MDL.push(y[i].to_owned());

            }

            //println!("Headers => {:?}",HEAD);
            //println!("Model => {:?}",MDL);

            let ec = db.createTable(name.clone(), HEAD, MDL);

            self.logger.update(ec, name.clone());

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

        fn createTable(&mut self, x:String, y:Vec<String>, z:Vec<String>) -> i32{
            
            if self.table.contains_key(&x){
                return -3;
            }

            let mut v= Table{
                name:x.clone(),
                headers:y,
                model:z,
                cells : Vec::new(),
                relations:Vec::new()
            };

            let _ = self.table.insert(x, v);
            return 3;

        }

        fn print(&self){
            let num = rand::thread_rng().gen_range(0..9);
            let mut name = String::from(" ");
            name.push_str(self.getName());
            name.push_str(" ");
            match num {
                //need to improve this piece of shit, too many repitions.
                1 => {println!("Tables of {} => {:?} ", &name.on_truecolor(0, 128, 128), &self.table.keys());},
                2 =>{println!("Tables of {} => {:?} ", &name.on_truecolor(128,0,128), &self.table.keys());},
                3 =>{println!("Tables of {} => {:?} ", &name.on_truecolor(64,64,128), &self.table.keys());},
                4 =>{println!("Tables of {} => {:?} ", &name.on_truecolor(43, 12, 28), &self.table.keys());},
                5 =>{println!("Tables of {} => {:?} ", &name.on_truecolor(91,17,138), &self.table.keys());},
                6 =>{println!("Tables of {} => {:?} ", &name.on_truecolor(128, 100 , 30), &self.table.keys());},
                7 =>{println!("Tables of {} => {:?} ", &name.on_truecolor(128,64,0), &self.table.keys());},
                _ => {println!("Tables of {} => {:?} ", &name.on_truecolor(0,64,0), &self.table.keys());},
            }

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

    fn looper() { // Not a movie reference

    }

    pub fn generateSession(){

        println!("");
        println!("{} : {} ", " dataBased ".on_white(), " Version 0.1 ".magenta());
        let mut workspaces: HashMap<String, Workspace>= HashMap::new();
        println!("Live session started: {}", &Utc::now().to_string());
        println!("");
        print!("Session Name? => ");
        io::stdout().flush().unwrap();
        
        let mut input = String::from("");
        let mut dir = String::new();
        let mut session_name = String::from("");

        let mut a_ws = String::new();
        let mut a_db = String::new();
        let mut a_tb = String::new();

        let mut b_ws = false;
        let mut b_db = false;
        let mut b_tb = false;

        let stdin = io::stdin();
        stdin.read_line(&mut session_name).unwrap();  

        session_name = session_name[0..session_name.len()-2].to_owned();

        println!("");
        dir.push_str(&session_name);
        


        let mut logger = init_Logger();

        loop {
            
            dir = String::new();
            dir.push_str(&session_name);
            
            if b_ws {
                dir.push_str(">");
                dir.push_str(&a_ws);
            }
            if b_db {
                dir.push_str(">");
                dir.push_str(&a_db);
            }
            if b_tb {
                dir.push_str(">");
                dir.push_str(&a_tb);
            }

            input = String::new();
            print!("{} => ", dir.green());
            io::stdout().flush().unwrap();
            stdin.read_line(&mut input).unwrap();

            input = input[0..input.len()-2].to_owned();

            let g = input.split(" ").collect::<Vec<&str>>();
            let len = g.len();

            //Filter input command by length
            match len {
                0=>{
                    logger.update(-1001, input);
                }
                1=>{
                    match input.to_lowercase().as_str(){
                        "" =>{
                            logger.update(-1001, input); //COMMAND isEmpty
                        }
                        "help" =>{
                            println!("Help is on the way!")
                        }
                        "status"=>{
                            for (_k,v) in &workspaces{
                                    v.print();
                            }
                        }
                        "exit" =>{
                            break;
                            //println!("Help is on the way!")
                        }
                        _ =>{
                            logger.update(-1000, input); //COMMAND DNE
                        }
                    }

                }
                2=>{

                    match g[0].to_lowercase().as_str(){

                        "unset" =>{

                            match g[1].to_lowercase().as_str(){
                                "all" =>{
                                    b_ws = false;
                                    b_db = false;
                                    b_tb = false;

                                    a_ws = String::new();
                                    a_db = String::new();
                                    a_tb = String::new();
                                }
                                "ws" =>{
                                    b_ws = false;
                                    b_db = false;
                                    b_tb = false;

                                    a_ws = String::new();
                                    a_db = String::new();
                                    a_tb = String::new();
                                }
                                "db" =>{
                                    b_db = false;
                                    b_tb = false;

                                    a_db = String::new();
                                    a_tb = String::new();
                                }
                                "tb"=>{
                                    b_tb = false;

                                    a_tb = String::new();
                                }
                                _ =>{
                                    logger.update(-1000, input);
                                }
                            }

                        }

                        _ =>{

                        }

                    }


                }
                3=>{

                    match g[0].to_lowercase().as_str(){

                        "set" =>{
                            match g[1].to_lowercase().as_str(){

                                "workspace" => {
                                    
                                    if workspaces.contains_key(g[2]){
                                        b_ws = true;
                                        a_ws = g[2].to_string();
                                    }else{
                                        logger.update(-1003, g[2].to_string())
                                    }
                                
                                }
                                _ =>{

                                }
                                }
                        }

                        "create" =>{

                            match g[1].to_lowercase().as_str(){

                                "workspace" => {

                                    if !workspaces.contains_key(g[2]){
                                    workspaces.insert(g[2].to_string(),create_workspace(g[2].to_string(), session_name.clone()));

                                    }else{
                                        logger.update(-1002, g[2].to_string())
                                    }


                                }
                                "database" =>{
                                    if !b_ws{
                                        logger.update(-4, g[2].to_owned())
                                    }else{
                                        let active = workspaces.get_mut(&a_ws).unwrap();
                                        active.addDB(g[2].to_string()) 
                                    }
                                }
                                _ =>{
                                    logger.update(-1000, g[1].to_string())
                                }



                            }

                        }
                        _ =>{

                        }

                    }

                }
                _ =>{
                    logger.update(-1000, input);
                }

            }

            println!()

        }

    }

}

fn main() {
    generateSession();

    let mut x = Vec::new();
    let mut y =  Vec::new();

    x.push(1);
    y.push("Moosa");

    let mut z: Vec<Box<dyn Debug>> = Vec::new();

    z.push(Box::new(x));
    z.push(Box::new(y));

    for i in z{
        println!("{:?}", i);
    }
    
}
