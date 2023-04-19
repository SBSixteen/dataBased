use dataBased::generateSession;
use std::{collections::HashMap, fmt::Debug};

pub mod dataBased {

    use std::{
        collections::HashMap,
        fmt::{Error, Debug},
        hash::Hash,
        io::{self, stdout, Write},
    };

    use chrono::{Utc, Local};
    use colored::Colorize;
    use rand::Rng;

    #[derive(Debug)]
    struct Logger {
        code: Vec<i32>,
        positives: bool,
    }

    #[derive(Debug)]
    pub struct Workspace {
        name: String,
        author: String,
        metadata: String,
        database: HashMap<String, Db>,
        logger: Logger,
    }

    #[derive(Debug)]
    pub struct Db {
        name: String,
        table: HashMap<String, Table>,
    }
    
    #[derive(Debug)]
    pub struct Table {
        name: String,
        model: Vec<String>,
        order: Vec<String>,
        cells: HashMap<String, Vec<Box<dyn Debug>>>,
        rows: i32,
        relations: Vec<Relation>,
    }

    #[derive(Debug)]
    pub struct Relation {
        table_name: String,
        col_name: String,
        
    }

    impl Logger {
        pub fn reset(&mut self) {
            self.code = Vec::new();
        }

        pub fn reveal(&mut self) -> usize {
            return self.code.clone().len();
        }

        pub fn update(&mut self, x: i32, y: String) {
            &self.code.push(x);
            self.throw(y);
        }

        fn throw(&mut self, y: String) {
            let x = self.getLen();
            let error_code = self.code[x - 1];

            if !self.positives && error_code > 0 {
                return;
            }

            let mut e_string = error_code.to_string();
            e_string.push_str(" ");

            println!();
            match error_code {
                -1 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_red(),
                        e_string.on_red(),
                        "Database named",
                        &y.cyan(),
                        "does not exist in the current workplace!"
                    );
                }
                -2 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Warning Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Database named",
                        &y.cyan(),
                        "already exists in workspace!"
                    );
                }
                -3 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Warning Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Table named",
                        &y.cyan(),
                        "already exists in this database!"
                    );
                }
                -4 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Warning Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create database ",
                        &y.cyan(),
                        "as no workspace is set in session!"
                    );
                }
                -5 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create table ",
                        &y.cyan(),
                        "as there are more columns than their assigned datatypes!"
                    );
                }
                -6 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create table ",
                        &y.cyan(),
                        "as there are more assigned datatypes than their columns!"
                    );
                }
                -7 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create table as",
                        &y.cyan(),
                        "is an invalid datatype!"
                    );
                }
                -8 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create table ",
                        &y.cyan(),
                        "as there are invalid datatypes declared!"
                    );
                }
                -9 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create table",
                        &y.cyan(),
                        "as there is no database set in the current session!"
                    );
                }
                -10 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create table",
                        &y.cyan(),
                        "as there is no workspace set in the current session!"
                    );
                }
                -11 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Cannot create table ",
                        &y.cyan(),
                        "as there are more assigned datatypes than their columns!"
                    );
                }
                -1000 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Warning Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Command",
                        &y.cyan(),
                        "does not exist!"
                    );
                }
                -1001 => {
                    println!(
                        "{}{}| {}",
                        " Warning Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "No Command typed."
                    );
                }
                -1002 => {
                    println!(
                        "{}{}| {} {} {}",
                        " Warning Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "Workspace",
                        &y.cyan(),
                        "already exists in current session!"
                    );
                }
                -1003  =>{
                    println!("{}{}| {} {} {}", " Warning Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Workspace", &y.cyan(), "does not exist in current session!");
                }
                -1004  =>{
                    println!("{}{}| {} {} {}", " Warning Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Database", &y.cyan(), "does not exist in current session!");
                }
                -1005  =>{
                    println!("{}{}| {} {} {}", " Warning Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Table", &y.cyan(), "does not exist in current session!");
                }
                -1007  =>{
                    println!("{}{}| {} {} {}", " Warning Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Amount of fields in", &y.cyan(), "do not match the input fields");
                }
                -1008  =>{
                    println!("{}{}| {} {} {}", " Warning Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"There is", &y.cyan(), "no table selected in this session!");
                }
                -1010  =>{
                    println!("{}{}| {} {} {}", " Warning Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"There are", &y.cyan(), "zero workspaces in this session!");
                }

                _ => {
                    println!(
                        "{}{}| {}",
                        " Undocumented Code ".on_truecolor(0, 0, 255),
                        e_string.to_string().on_truecolor(0, 0, 255),
                        "No definition found for this error!"
                    );
                }
            }
        }

        fn getLen(&mut self) -> usize {
            return self.code.len();
        }

        pub fn toggle(&mut self) {
            self.positives = !self.positives;
        }
    }

    impl Workspace {
        pub fn getName(&self) -> &String {
            return &self.name;
        }

        pub fn getAuthor(&self) -> &String {
            return &self.author;
        }

        pub fn getMetadata(&self) -> &String {
            return &self.metadata;
        }

        pub fn getDB(&self) -> &HashMap<String, Db> {
            return &self.database;
        }

        pub fn throwUC(&mut self) {
            //Force throws undocumented error code exception!
            self.logger.update(-99999, "".to_owned());
        }

        pub fn toggleLogger(&mut self) {
            self.logger.toggle();
        }

        fn getDBNames(&self) -> String {
            let g = &self.database;

            let mut x = String::from("[");

            if g.len() == 0 {
                return String::from("[]");
            }

            for (k, v) in g {
                x.push_str(v.getName());
                x.push_str(", ");
            }
            x = x[0..x.len() - 2].to_owned();
            x.push_str("]");

            return x;
        }

        pub fn print(&self) {
            println!();
            println!("Name => {:?}", self.getName());
            println!("Author => {:?}", self.getAuthor());
            println!("Metadata => {:?}", self.getMetadata());
            println!("Databases => {:?}", self.getDBNames());
            for (k, v) in &self.database {
                let x = v;
                x.print();
            }
        }

        pub fn addDB(&mut self, x: String) {
            if self.database.contains_key(&x) {
                self.logger.update(-3, x);
                return;
            }

            let y = Db {
                name: x.clone(),
                table: HashMap::new(),
            };

            let _ = &self.database.insert(x, y);
        }

        pub fn fetchDB(&mut self) -> &mut HashMap<String, Db> {
            return &mut self.database;
        }

        pub fn fetchDB_name(&mut self, x: String) -> bool {
            if self.database.contains_key(&x) {
                return true;
            } else {
                self.logger.update(-1, x);
                return false;
            }
        }

        pub fn createTable(&mut self, DB: String, name: String, headers: String, model: String) {
            let check_DB = self.fetchDB_name(DB.clone());

            if !check_DB {
                return;
            }

            let mut temp = self.fetchDB();
            let db = temp.get_mut(&DB).unwrap();

            let x = headers.split(",").collect::<Vec<_>>();
            let y = model.split(",").collect::<Vec<_>>();

            let mut HEAD = Vec::new();
            let mut MDL = Vec::new();

            //println!("{:?}",x);
            //println!("{:?}",y);

            for i in 0..x.len() {
                HEAD.push(x[i].to_owned());
                MDL.push(y[i].to_owned());
            }

            //println!("Headers => {:?}",HEAD);
            //println!("Model => {:?}",MDL);

            let ec = db.createTable(name.clone(), HEAD, MDL);

            self.logger.update(ec, name.clone());
        }
    }



    impl Db {
        fn getName(&self) -> &String {
            return &self.name;
        }

        fn getTables(&self) -> &HashMap<String, Table> {
            return &self.table;
        }

        fn createTable(&mut self, x: String, y: Vec<String>, z: Vec<String>) -> i32 {
            if self.table.contains_key(&x) {
                return -3;
            }

            let mut v = Table {
                name: x.clone(),
                model: z,
                order:y,
                cells: HashMap::new(),
                rows:0,
                relations: Vec::new(),
            };

            let _ = self.table.insert(x, v);
            return 3;
        }

        fn print(&self) {
            let num = rand::thread_rng().gen_range(0..9);
            let mut name = String::from(" ");
            name.push_str(self.getName());
            name.push_str(" ");
            match num {
                //need to improve this piece of shit, too many repitions.
                1 => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(0, 128, 128),
                        &self.table.keys()
                    );
                }
                2 => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(128, 0, 128),
                        &self.table.keys()
                    );
                }
                3 => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(64, 64, 128),
                        &self.table.keys()
                    );
                }
                4 => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(43, 12, 28),
                        &self.table.keys()
                    );
                }
                5 => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(91, 17, 138),
                        &self.table.keys()
                    );
                }
                6 => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(128, 100, 30),
                        &self.table.keys()
                    );
                }
                7 => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(128, 64, 0),
                        &self.table.keys()
                    );
                }
                _ => {
                    println!(
                        "Tables of {} => {:?} ",
                        &name.on_truecolor(0, 64, 0),
                        &self.table.keys()
                    );
                }
            }
        }
    }

    pub fn create_workspace(x: String, y: String) -> Workspace {
        let mut meta = String::from("");

        meta.push_str("Date Created : ");        

        meta.push_str(&Local::now().to_string());

        let x = Workspace {
            name: x,
            author: y,
            metadata: meta,
            database: HashMap::new(),
            logger: init_Logger(),
        };

        return x;
    }

    fn init_Logger() -> Logger {
        let mut L = Logger {
            code: Vec::new(),
            positives: false,
        };

        return L;
    }

    fn looper() { // Not a movie reference
    }

    pub fn generateSession() {
        println!("");
        println!(
            "{} : {} ",
            " dataBased ".on_white(),
            " Version 0.1 ".magenta()
        );
        let mut workspaces: HashMap<String, Workspace> = HashMap::new();
        println!("Live session started: {}", &Local::now().to_string());
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

        session_name = session_name[0..session_name.len() - 2].to_owned();

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

            input = input[0..input.len() - 2].to_owned();

            let g = input.split(" ").collect::<Vec<&str>>();
            let len = g.len();

            //Filter input command by length
            match len {
                0 => {
                    logger.update(-1001, input);
                }
                1 => {
                    match input.to_lowercase().as_str() {
                        "" => {
                            logger.update(-1001, input); //COMMAND isEmpty
                        }
                        "help" => {
                            println!("Help is on the way!")
                        }
                        "print" =>{

                            if b_tb{
                                let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                                println!("{:?}", temp.order);
                                let len = temp.rows;
                                let order = &temp.order;

                                for i in 0 as usize..len as usize{
                                    print!("{} => ", i);

                                    for j in order{

                                        let temp_hs = temp.cells.get(j).unwrap();
                                        print!("{:?}, ", temp_hs[i]);

                                    }
                                    println!();

                                }
                                

                            }else{
                                logger.update(-1008, "".to_owned());
                            }
                        }
                        "status" => {
                            for (_k, v) in &workspaces {
                                v.print();
                            }

                            if workspaces.len() == 0 {
                                logger.update(-1010, "".to_owned());
                            }
                        }
                        "exit" => {
                            break;
                            //println!("Help is on the way!")
                        }
                        _ => {
                            logger.update(-1000, input); //COMMAND DNE
                        }
                    }
                }
                2 => match g[0].to_lowercase().as_str() {
                    "unset" => match g[1].to_lowercase().as_str() {
                        "all" => {
                            b_ws = false;
                            b_db = false;
                            b_tb = false;

                            a_ws = String::new();
                            a_db = String::new();
                            a_tb = String::new();
                        }
                        "ws" => {
                            b_ws = false;
                            b_db = false;
                            b_tb = false;

                            a_ws = String::new();
                            a_db = String::new();
                            a_tb = String::new();
                        }
                        "db" => {
                            b_db = false;
                            b_tb = false;

                            a_db = String::new();
                            a_tb = String::new();
                        }
                        "tb" => {
                            b_tb = false;

                            a_tb = String::new();
                        }
                        _ => {
                            logger.update(-1011, input);
                        }
                    },

                    _ => {}
                },
                3 => match g[0].to_lowercase().as_str() {
                    "insert" => match g[1].to_lowercase().as_str() {
                        "record" => {

                            let temp = g[2].split(":").collect::<Vec<&str>>(); 
                            if temp.len() != workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap().model.len() {
                                //println!("{}","Error: Record length does not match table model".red());
                                logger.update(-1007, a_tb.to_owned());
                                continue;
                            }

                            let mut tb = workspaces.get_mut(&a_ws).unwrap().database.get_mut(&a_db).unwrap().table.get_mut(&a_tb).unwrap();
                            
                            let order = tb.order.clone();
                            let model = tb.model.clone();
                            let cells = &mut tb.cells;
                            let rows = tb.rows;

                            let len = model.clone().len() as usize;
                            //datacheck


                            for i in 0..len{

                                let mut t = cells.get_mut(&order[i]).unwrap(); 
                                let datatype = model[i].clone().to_lowercase();

                                match datatype.as_str(){

                                    "string" => {
                                        t.push(Box::new(temp[i].to_owned()));
                                    }
                                    "i32" => {
                                        t.push(Box::new(temp[i].parse::<i32>().unwrap()));
                                    }
                                    "integer" => {
                                        t.push(Box::new(temp[i].parse::<i32>().unwrap()));
                                    }
                                    "i64" => {
                                        t.push(Box::new(temp[i].parse::<i64>().unwrap()));
                                    }
                                    "f32" => {
                                        t.push(Box::new(temp[i].parse::<f32>().unwrap()));
                                    }
                                    "float" => {
                                        t.push(Box::new(temp[i].parse::<f64>().unwrap()));
                                    }
                                    "bool" => {
                                        t.push(Box::new(temp[i].parse::<bool>().unwrap()));
                                    }
                                    "boolean" => {
                                        t.push(Box::new(temp[i].parse::<bool>().unwrap()));
                                    }

                                    _=>{

                                    }

                                }                               

                            }

                            tb.rows += 1;


                            /* 
                            if b_tb {
                                let ws = workspaces.get(&a_ws).unwrap();
                                let db = ws.database.get(&a_db).unwrap();
                                let tb = db.table.get(&a_tb).unwrap();

                                let mut record: HashMap<String, Box<dyn Debug>> = HashMap::new();

                                for (k, v) in &tb.cells {
                                    let mut input = String::new();
                                    print!("{} => ", k.green());
                                    io::stdout().flush().unwrap();
                                    stdin.read_line(&mut input).unwrap();

                                    input = input[0..input.len() - 2].to_owned();

                                    let mut cell: Box<dyn Debug> = Box::new(input);

                                    match v[0].downcast_ref::<String>() {
                                        Some(_x) => {
                                            cell = Box::new(input);
                                        }
                                        None => {
                                            match v[0].downcast_ref::<i32>() {
                                                Some(_x) => {
                                                    cell = Box::new(input.parse::<i32>().unwrap());
                                                }
                                                None => {
                                                    match v[0].downcast_ref::<f32>() {
                                                        Some(_x) => {
                                                            cell = Box::new(input.parse::<f32>().unwrap());
                                                        }
                                                        None => {
                                                            match v[0].downcast_ref::<bool>() {
                                                                Some(_x) => {
                                                                    cell = Box::new(input.parse::<bool>().unwrap());
                                                                }
                                                                None => {}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    record.insert(k.to_string(), cell);
                                }

                                let mut tb = workspaces.get_mut(&a_ws).unwrap().database.get_mut(&a_db).unwrap().table.get_mut(&a_tb).unwrap();

                                for (k, v) in &record {
                                    tb.cells.get_mut(k).unwrap().push(v);
                                }
                            } else {
                                logger.update(-1006, input);
                            }  */ 
                        }
                        _=>{

                        }
                    }
                    "set" => match g[1].to_lowercase().as_str() {
                        "workspace" => {
                            if workspaces.contains_key(g[2]) {
                                b_ws = true;
                                a_ws = g[2].to_string();
                            } else {
                                logger.update(-1003, g[2].to_string())
                            }
                        }
                        "database" => {
                            if b_ws {

                                let ws = workspaces.get(&a_ws).unwrap();
                                if ws.database.contains_key(g[2]){
                                    b_db = true;
                                    a_db = g[2].to_string();
                                }else{
                                    logger.update(-1004, g[2].to_string())
                                }

                            } else {
                                logger.update(-1003, "".to_string())
                            }
                        }
                        "table" => {
                            if b_db && b_ws {

                                let ws = workspaces.get(&a_ws).unwrap();
                                let db = ws.database.get(&a_db).unwrap();
                                if db.table.contains_key(g[2]){
                                    b_tb = true;
                                    a_tb = g[2].to_string();
                                }else{
                                    logger.update(-1005, g[2].to_string())
                                }

                            } else {
                                logger.update(-1004, g[2].to_string())
                            }
                        }
                        _ => {}
                    },

                    "create" => match g[1].to_lowercase().as_str() {
                        "workspace" => {
                            if !workspaces.contains_key(g[2]) {
                                workspaces.insert(
                                    g[2].to_string(),
                                    create_workspace(g[2].to_string(), session_name.clone()),
                                );
                            } else {
                                logger.update(-1002, g[2].to_string())
                            }
                        }
                        "database" => {
                            if !b_ws {
                                logger.update(-4, g[2].to_owned())
                            } else {
                                let active = workspaces.get_mut(&a_ws).unwrap();
                                active.addDB(g[2].to_string())
                            }
                        }
                    

                        _ => logger.update(-1000, g[1].to_string()),
                    },
                    _ => {}
                },
                4=>{
                    
                }

                5=>{
                    
                    match g[0].to_lowercase().as_str(){

                        "create"=>{

                            match g[1].to_lowercase().as_str(){
                                
                                "table"=>{

                                    let x = g[3].split(":").collect::<Vec<&str>>();
                                    let y = g[4].split(":").collect::<Vec<&str>>();

                                    if x.len() != y.len(){
                                        if x.len() > y.len(){
                                            logger.update(-5, g[2].to_owned());
                                        }else{
                                            logger.update(-6,g[2].to_owned());
                                        }
                                    }else{

                                        let mut count = 0;

                                        for i in 0..y.len(){

                                            match y[i].to_lowercase().as_str(){

                                                "integer"=>{
                                                    count+=1;
                                                }
                                                "int"=>{
                                                    count+=1;
                                                }
                                                "i32"=>{
                                                    count+=1;
                                                }
                                                "number"=>{
                                                    count+=1;
                                                }
                                                "string"=>{
                                                    count+=1;
                                                }
                                                "word"=>{
                                                    count+=1;
                                                }
                                                "str"=>{
                                                    count+=1;
                                                }
                                                "char"=>{
                                                    count+=1;
                                                }
                                                "letter"=>{
                                                    count+=1;
                                                }
                                                "float"=>{
                                                    count+=1;
                                                }
                                                "f64"=>{
                                                    count+=1;
                                                }
                                                "f64"=>{
                                                    count+=1;
                                                }
                                                "bool"=>{
                                                    count+=1;
                                                }
                                                "boolean"=>{
                                                    count+=1;
                                                }
                                                _=>{
                                                    logger.update(-7, g[2].to_owned());
                                                }

                                            }

                                        }

                                        if count == x.len(){                                        
                                            
                                            let mut types = Vec::new();
                                            let mut orders = Vec::new();
                                            let mut data : HashMap<String, Vec<Box<dyn Debug>>> = HashMap::new();
                                           
                                            for i in 0..y.len(){
                                                types.push(y[i].to_owned());
                                                orders.push(x[i].to_owned());
                                                data.insert(x[i].to_owned(), Vec::new());
                                            }
                                            
                                            if b_ws{
                                                let mut k = workspaces.get_mut(&a_ws).unwrap();
                                                if b_db{

                                                    let mut d = k.database.get_mut(&a_db).unwrap(); 

                                                    let mut instance = Table{
                                                        name:g[2].to_owned(),
                                                        model:types,
                                                        order:orders,
                                                        cells:data,
                                                        rows:0,
                                                        relations:Vec::new()
                                                    };

                                                    d.table.insert(g[2].to_string(), instance);

                                                }else{
                                                    logger.update(-9, g[2].to_owned())
                                                }
                                        }else{
                                            logger.update(-10, g[2].to_owned());
                                        }

                                        }
                                        else{
                                            logger.update(-8,g[2].to_owned());
                                        }

                                    }

                                }
                                _ =>{
                                    logger.update(-1000, "".to_owned());
                                }
                            }

                            

                        }
                        _=>{
                            logger.update(-1000, "".to_owned());
                        }

                    }

                }
                _=>{
                    logger.update(-1000, "".to_owned())
                }
            }

            println!()
        }
    }
}


fn main() {
    
    generateSession();

}

fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}

