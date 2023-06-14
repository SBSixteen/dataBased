use dataBased::generateSession;
use std::{collections::HashMap, fmt::Debug, fs};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;




pub mod dataBased {

    use std::{
        collections::HashMap,
        fmt::{Error, Debug},
        hash::Hash,
        io::{self, stdout, Write, BufRead, Bytes}, fs::{self, File, read_to_string}, any::Any, path::Path,
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
        cells: HashMap<String, Vec<Box<dyn Any + Send>>>, //nabeel
        rows: i32,
        relations: Vec<Relation>,
    }

    #[derive(Debug)]
    pub struct Relation {
        table_name: String,
        col_name: String,
        
    }

    //nabeel
    fn apply_aggregate_function(
        function_name: &str,
        column_name: &str,
        table: &Table,
        groups: &HashMap<String, Vec<usize>>,
    ) {
        let column_values: Vec<String> = table.cells
            .get(column_name)
            .unwrap()
            .iter()
            .map(|v| v.downcast_ref::<String>().map(|s| s.to_string())
                .or_else(|| v.downcast_ref::<i32>().map(|i| i.to_string()))
                .or_else(|| v.downcast_ref::<f32>().map(|f| f.to_string()))
                .or_else(|| v.downcast_ref::<bool>().map(|b| b.to_string()))
                .or_else(|| v.downcast_ref::<char>().map(|c| c.to_string()))
                .unwrap_or_else(|| format!("{:?}", v))
            ).collect();
    
        for (group_value, indices) in groups {
            match function_name {
                "count" => {
                    // Simply count the number of elements in the group
                    let count = indices.len();
                    println!("For group {}: COUNT = {}", group_value, count);
                }
                //the ones below only work if the column is of data type numeric
                "sum" => {

                    let sum: i32 = indices.iter()
                        .map(|&i| column_values[i].parse::<i32>().unwrap_or(0))  // Adapt this to handle other types
                        .sum();
                    println!("For group {}: SUM = {}", group_value, sum);
                }
                "avg" => {

                    let avg: f64 = indices.iter()
                        .map(|&i| column_values[i].parse::<f64>().unwrap_or(0.0))  // Adapt this to handle other types
                        .sum::<f64>() / indices.len() as f64;
                    println!("For group {}: AVG = {}", group_value, avg);
                }
                "min" => {

                    let min: i32 = indices.iter()
                        .map(|&i| column_values[i].parse::<i32>().unwrap_or(0))  // Adapt this to handle other types
                        .min()
                        .unwrap_or(0);
                    println!("For group {}: MIN = {}", group_value, min);
                }
                "max" => {

                    let max: i32 = indices.iter()
                        .map(|&i| column_values[i].parse::<i32>().unwrap_or(0))  // Adapt this to handle other types
                        .max()
                        .unwrap_or(0);
                    println!("For group {}: MAX = {}", group_value, max);
                }
                _ => {
                    println!("Unknown function: {}", function_name);
                }
            }
        }
    }

        //nabeel
    fn import_table_from_csv(db: &mut Db, file_name: &str) -> io::Result<()> {
        println!("Importing table from CSV file: {}", file_name);   
        let file_path = Path::new(file_name);
        println!("File path: {:?}", file_path);
        let file = File::open(&file_path)?;
        let reader = io::BufReader::new(file);
    
        let mut lines = reader.lines();
        let table_name = file_path.file_stem().unwrap().to_str().unwrap().to_owned();
    
        // First line: column names (order)
        if let Some(Ok(line)) = lines.next() {
            let order: Vec<String> = line.split(',')
                .map(|s| s.trim().to_string())
                .map(|s| if s.starts_with('\u{feff}') { s.replacen('\u{feff}', "", 1) } else { s })
                .collect();
            // Second line: datatypes (model)
            if let Some(Ok(line)) = lines.next() {
                let model: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();  
                let mut table = Table {
                    name: table_name.clone(),
                    model,
                    order: order.clone(),
                    cells: HashMap::new(),
                    rows: 0,
                    relations: Vec::new(),
                }; 
                // Print table name
                println!("Table name: {}", table.name);
                println!("Table columns: {:?}", table.order);
                println!("Table datatypes: {:?}", table.model);
                // All other lines: entries
                for line in lines {
                    if let Ok(line) = line {
                        let values: Vec<&str> = line.split(',').collect();
                        for (i, value) in values.iter().enumerate() {
                            let cell = match &table.model[i][..] {
                                "i32" => Box::new(value.trim().parse::<i32>().unwrap()) as Box<dyn Any + Send>,
                                "f32" => Box::new(value.trim().parse::<f32>().unwrap()) as Box<dyn Any + Send>,
                                "bool" => Box::new(value.trim().parse::<bool>().unwrap()) as Box<dyn Any + Send>,
                                "char" => Box::new(value.trim().parse::<char>().unwrap()) as Box<dyn Any + Send>,
                                _ => Box::new(value.trim().to_string()) as Box<dyn Any + Send>,
                            };
                            table.cells.entry(order[i].clone()).or_insert_with(Vec::new).push(cell);
                        }
                        table.rows += 1;
                    }
                }
                db.table.insert(table_name, table);
            }
        }
        Ok(())
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

                -12 => {
                    println!(
                        "{}{}| {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "No workspace set in this session!",
                    );
                }

                -13 => {
                    println!(
                        "{}{}| {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "No database set in this session!",
                    );
                }
                -14 => {
                    println!(
                        "{}{}| {}",
                        " Error Code ".on_bright_yellow(),
                        e_string.to_string().on_bright_yellow(),
                        "No table set in this session!",
                    );
                }
                -15  =>{
                    println!("{}{}| {} {} {}", " Error Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Export type", &y.cyan(), "is not supportd by dataBased!");
                }
                -16 =>{
                    println!("{}{}| {} {} {}", " Error Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Object Type:", &y.cyan(), "cannot be saved by dataBased!");
                }
                -17 =>{
                    println!("{}{}| {} {} {}", " Error Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Cannot Delete Table:", &y.cyan(), "as it does not exist!");
                }
                -18 =>{
                    println!("{}{}| {} {} {}", " Error Code ".on_bright_yellow() , e_string.to_string().on_bright_yellow() ,"Cannot Import from Type:", &y.cyan(), "as it is invalid!");
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

            let mut g = input.split(" ").collect::<Vec<&str>>();
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

                        "cleartable" =>{
                            if b_tb{
                                let temp = workspaces.get_mut(&a_ws).unwrap().database.get_mut(&a_db).unwrap().table.get_mut(&a_tb).unwrap();
                                temp.cells = HashMap::new();
                                temp.rows = 0;
                            }else {
                                logger.update(-9999, String::new())
                            }
                        }
                        "print" => {
                            if (b_tb) {
                                let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                                let order = &temp.order;
                                let total_rows = temp.rows as usize;

                                for index in 0..total_rows {
                                    for key in order {
                                        if let Some(value) = temp.cells.get(key).and_then(|v| v.get(index)) {
                                            let return_value = value.downcast_ref::<String>().map(|s| s.to_string())
                                                .or_else(|| value.downcast_ref::<i32>().map(|i| i.to_string()))
                                                .or_else(|| value.downcast_ref::<f64>().map(|f| f.to_string()))
                                                .or_else(|| value.downcast_ref::<bool>().map(|b| b.to_string()))
                                                .or_else(|| value.downcast_ref::<char>().map(|c| c.to_string()))
                                                .unwrap_or_else(|| format!("{:?}", value));

                                            print!("{} : {}, ", key, return_value);
                                        }                
                                    }
                                    println!();
                                }
                            }
                        }
                        /* "print" =>{

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
                        } */
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
                    //insert it here
                    "export"=> {
                        
                        let mut count =0;

                        if !b_ws{

                            logger.update(-12, "".to_owned());
                            count+=1;
                        }
                        if !b_db{

                            logger.update(-13, "".to_owned());
                            count+=1;
                        }
                        if !b_tb{

                            logger.update(-14, "".to_owned());
                            count+=1;
                        }

                        if count!=0{
                            continue;
                        }

                        match g[1].to_lowercase().as_str() {

                        "json" =>{

                            let mut data = String::from("//");

                            let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                            let mut vec = Vec::new();

                            for i in &temp.order{
                                vec.push(temp.cells.get(i.as_str()).unwrap());
                            }

                            for i in &temp.model{
                                data.push_str(i);
                                data.push_str(":")
                            }

                            data = data[0..data.len()-1].to_owned();
                            data.push_str("\r\n");

                            data.push_str("//");

                            for i in &temp.order{
                                data.push_str(i);
                                data.push_str(":")
                            }

                            data = data[0..data.len()-1].to_owned();
                            data.push_str("\r\n");

                            data.push_str("[\r\n");

                            for i in 0 as usize..temp.rows as usize{
                                data.push_str("{ \r\n");
                                for j in 0..vec.len(){

                                    data.push_str(r#"""#);
                                    data.push_str(&temp.order[j]);
                                    data.push_str(r#"""#);
                                    data.push_str(":");

                                    let temp_dtype = temp.model[j].to_lowercase().clone();

                                    if temp_dtype == "string" || temp_dtype == "word" || temp_dtype == "str"{
                                        let x = vec[j].get(i).unwrap().downcast_ref::<String>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x.replace(" ", "%20")));
                                    }
                                    if temp_dtype == "integer" || temp_dtype == "int" || temp_dtype == "i32" || temp_dtype == "number"{
                                        let x = vec[j].get(i).unwrap().downcast_ref::<i32>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "char" || temp_dtype == "letter" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<String>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "float" || temp_dtype == "f64" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<f64>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "bool" || temp_dtype == "boolean" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<bool>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if j != vec.len()-1{
                                    data.push_str(",");
                                    }
                                }


                                if i == temp.rows as usize -1{
                                    data.push_str("\r\n}\r\n");
                                }else{
                                    data.push_str("\r\n},\r\n");
                                }
                            }

                            data.push_str("]");

                            let mut path = String::from("./");
                            path.push_str(&a_tb);
                            path.push_str(".js");

                            fs::write(&path, data).expect("Unable to write file");
                        }

                        "xml"=>{
                            
                            let mut data = String::from("<");
                            data.push_str(&a_tb);
                            data.push_str(">\r\n");

                            let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                            let mut vec = Vec::new();

                            let mut data = String::from("<dtypes = ");
                            for i in &temp.model{
                                data.push_str(i);
                                data.push_str(":")
                            }

                            data = data[0..data.len()-1].to_owned();
                            data.push_str("/>\r\n");

                            data.push_str("<colnames = ");

                            for i in &temp.order{
                                data.push_str(i);
                                data.push_str(":")
                            }

                            data = data[0..data.len()-1].to_owned();
                            data.push_str("/>\r\n");

                            data.push_str("<");
                            data.push_str(&a_tb);
                            data.push_str(">\r\n");

                            for i in &temp.order{
                                vec.push(temp.cells.get(i.as_str()).unwrap());
                            }
                            
                            for i in 0 as usize..temp.rows as usize{
                                data.push_str("<Record");
                                for j in 0..vec.len(){

                                    data.push_str(" ");
                                    data.push_str(&temp.order[j]);
                                    data.push_str("=");

                                    //println!("{:?}", temp);
                                    let temp_dtype = temp.model[j].to_lowercase().clone();

                                    if temp_dtype == "string" || temp_dtype == "word" || temp_dtype == "str"{
                                        let x = vec[j].get(i).unwrap().downcast_ref::<String>().unwrap();
                                        data.push_str(&format!("{:?}",x.replace(" ", "%20")));
                                    }
                                    if temp_dtype == "integer" || temp_dtype == "int" || temp_dtype == "i32" || temp_dtype == "number"{
                                        let x = vec[j].get(i).unwrap().downcast_ref::<i32>().unwrap();

                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "char" || temp_dtype == "letter" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<String>().unwrap();
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "float" || temp_dtype == "f64" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<f64>().unwrap();
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "bool" || temp_dtype == "boolean" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<bool>().unwrap();
                                        data.push_str(&format!("{:?}",x));
                                    }

                                }

                                    data.push_str("/>\r\n");

                            }

                            data.push_str("<");
                            data.push_str(&a_tb);
                            data.push_str("/>");

                            let mut path = String::from("./");
                            path.push_str(&a_tb);
                            path.push_str(".xml");

                            fs::write(&path, data).expect("Unable to write file");

                        }

                        "csv"=>{
                            
                            let mut data = String::from("//");

                            let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                            let mut vec = Vec::new();

                            for i in &temp.model{
                                data.push_str(i);
                                data.push_str(":")
                            }

                            data = data[0..data.len()-1].to_owned();
                            data.push_str("\r\n");

                            data.push_str("//");

                            for i in &temp.order{
                                data.push_str(i);
                                data.push_str(":")
                            }

                            data = data[0..data.len()-1].to_owned();
                            data.push_str("\r\n");

                            for i in &temp.order{
                                vec.push(temp.cells.get(i.as_str()).unwrap());
                            }

                            for i in 0 as usize..temp.rows as usize{
                                for j in 0..vec.len(){
                                    
                                    let temp_dtype = temp.model[j].to_lowercase().clone();

                                    if temp_dtype == "string" || temp_dtype == "word" || temp_dtype == "str"{
                                        let x = vec[j].get(i).unwrap().downcast_ref::<String>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "integer" || temp_dtype == "int" || temp_dtype == "i32" || temp_dtype == "number"{
                                        let x = vec[j].get(i).unwrap().downcast_ref::<i32>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "char" || temp_dtype == "letter" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<String>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "float" || temp_dtype == "f64" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<f64>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if temp_dtype == "bool" || temp_dtype == "boolean" {
                                        let x = vec[j].get(i).unwrap().downcast_ref::<bool>().unwrap();
                                        println!("{}",x);
                                        data.push_str(&format!("{:?}",x));
                                    }
                                    if j != vec.len()-1{
                                    data.push_str(",");
                                    }
                                }

                                    data.push_str("\r\n");

                            }

                            let mut path = String::from("./");
                            path.push_str(&a_tb);
                            path.push_str(".csv");

                            fs::write(&path, data).expect("Unable to write file");

                        }

                        _ =>{

                            logger.update(-1013,g[1].to_string());

                        }}
                        

                    },
                    "save" =>
                        match g[1].to_lowercase().as_str(){
                            "workspace" =>{
                                let current = workspaces.get(&a_ws).unwrap().clone();
                                current.print();
                            }

                            _ =>{
                                logger.update(-16, g[1].to_owned())
                            }
                        }
        
                    _ => {}
                },
                3 => match g[0].to_lowercase().as_str() {
                    "print" => match g[1].to_lowercase().as_str() {
                        "orderby" => {
                            if b_tb { // if table is selected
                                let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                                let order = &temp.order;
                                let column_name = &g[2].to_string();
                                let mut found = false;

                            for i in order {
                                if i == column_name {
                                    found = true;
                                    // sort by column name (order and print)
                                    let mut rows: Vec<_> = temp.cells.get(column_name).unwrap().iter().enumerate().collect();
                                    rows.sort_by(|a, b| {
                                    let a_val = a.1.downcast_ref::<String>().map(|s| s.to_string())
                                    .or_else(|| a.1.downcast_ref::<i32>().map(|i| i.to_string()))
                                    .or_else(|| a.1.downcast_ref::<f32>().map(|f| f.to_string()))
                                    .or_else(|| a.1.downcast_ref::<bool>().map(|b| b.to_string()))
                                    .or_else(|| a.1.downcast_ref::<char>().map(|c| c.to_string()))
                                    
                                    .unwrap_or_else(|| format!("{:?}", a.1));
                                    let b_val = b.1.downcast_ref::<String>().map(|s| s.to_string())
                                    .or_else(|| b.1.downcast_ref::<i32>().map(|i| i.to_string()))
                                    .or_else(|| b.1.downcast_ref::<f32>().map(|f| f.to_string()))
                                    .or_else(|| b.1.downcast_ref::<bool>().map(|b| b.to_string()))
                                    .or_else(|| b.1.downcast_ref::<char>().map(|c| c.to_string()))
                                    .unwrap_or_else(|| format!("{:?}", b.1));
                                    a_val.cmp(&b_val)
                                    });

                                for (index, _) in rows {
                                    //println!("Row #{}:", index + 1);
                                    for key in order {
                                        if let Some(value) = temp.cells.get(key).and_then(|v| v.get(index)) {
                                            let formatted_value = value.downcast_ref::<String>().map(|s| s.to_string())
                                            .or_else(|| value.downcast_ref::<i32>().map(|i| i.to_string()))
                                            .or_else(|| value.downcast_ref::<bool>().map(|b| b.to_string()))
                                            .or_else(|| value.downcast_ref::<f32>().map(|f| f.to_string()))
                                            .or_else(|| value.downcast_ref::<char>().map(|c| c.to_string()))
                                            // Add more types if needed
                                            .unwrap_or_else(|| format!("{:?}", value));

                                            print!("{}: {} ", key, formatted_value);
                                        }
                                    }
                                    println!();
                                }
                                break;
                                }
                            }

                                if !found {
                                    println!("Invalid column name"); //nabeel check this
                                }
                            }
                        }
                        "orderbyrev" => { //nabeel
                            if b_tb { // if table is selected
                                let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                                let order = &temp.order;
                                let column_name = &g[2].to_string();
                                let mut found = false;
                    
                                for i in order {
                                    if i == column_name {
                                        found = true;
                                        // sort by column name (order and print)
                                        let mut rows: Vec<_> = temp.cells.get(column_name).unwrap().iter().enumerate().collect();
                                        rows.sort_by(|a, b| {
                                            let a_val = a.1.downcast_ref::<String>().map(|s| s.to_string())
                                                .or_else(|| a.1.downcast_ref::<i32>().map(|i| i.to_string()))
                                                .or_else(|| a.1.downcast_ref::<f32>().map(|f| f.to_string()))
                                                .or_else(|| a.1.downcast_ref::<bool>().map(|b| b.to_string()))
                                                .or_else(|| a.1.downcast_ref::<char>().map(|c| c.to_string()))
                                                .unwrap_or_else(|| format!("{:?}", a.1));
                                            let b_val = b.1.downcast_ref::<String>().map(|s| s.to_string())
                                                .or_else(|| b.1.downcast_ref::<i32>().map(|i| i.to_string()))
                                                .or_else(|| b.1.downcast_ref::<f32>().map(|f| f.to_string()))
                                                .or_else(|| b.1.downcast_ref::<bool>().map(|b| b.to_string()))
                                                .or_else(|| b.1.downcast_ref::<char>().map(|c| c.to_string()))
                                                .unwrap_or_else(|| format!("{:?}", b.1));
                                            a_val.cmp(&b_val)
                                        });
                    
                                        for (index, _) in rows.iter().rev() {
                                            //println!("Row #{}:", index + 1);
                                            for key in order {
                                                if let Some(value) = temp.cells.get(key).and_then(|v| v.get(*index)) {
                                                    let formatted_value = value.downcast_ref::<String>().map(|s| s.to_string())
                                                        .or_else(|| value.downcast_ref::<i32>().map(|i| i.to_string()))
                                                        .or_else(|| value.downcast_ref::<bool>().map(|b| b.to_string()))
                                                        .or_else(|| value.downcast_ref::<f32>().map(|f| f.to_string()))
                                                        .or_else(|| value.downcast_ref::<char>().map(|c| c.to_string()))
                                                        .unwrap_or_else(|| format!("{:?}", value));
                    
                                                    print!(" Row # {} => {}: {} ", index + 1, key, formatted_value);
                                                }
                                            }
                                            println!();
                                        }
                                        break;
                                    }
                                }
                    
                                if !found {
                                    println!("Invalid column name");
                                }
                            }
                        }
                        _ => {
                            // Handle other print options
                        }
                    }
                    "insert" => match g[1].to_lowercase().as_str() {
                        "record" => {

                            if !b_tb{
                                logger.update(-14, "".to_owned());
                                continue;
                            }

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
                                        t.push(Box::new(temp[i].replace("%20", " ").to_owned()));
                                    }
                                    "char" => {
                                        let temp_char = temp[i][0..temp[i].len()].to_owned();
                                        t.push(Box::new(temp_char));
                                    }
                                    "i32" => {
                                        t.push(Box::new(temp[i].parse::<i32>().unwrap()));
                                    }
                                    "integer" => {
                                        t.push(Box::new(temp[i].parse::<i32>().unwrap()));
                                    }
                                    "int" => {
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
                                    "boolean" => {
                                        let test = temp[i].to_lowercase();
                                        if test == "true"{
                                            t.push(Box::new(true));
                                        }else{
                                            t.push(Box::new(false));
                                        }
                                    }

                                    _=>{
                                        //invalid datatype return errors
                                    }

                                }                               

                            }

                            tb.rows += 1;


                            
                        }
                        _=>{

                        }
                    }
                    "set" => {
                    
                    if g[1].to_lowercase().as_str() == "db"{
                        g[1] = "database"
                    }

                    if g[1].to_lowercase().as_str() == "ws"{
                        g[1] = "workspace"
                    }
                    
                    match g[1].to_lowercase().as_str() {
                        "workspace" => {
                            if workspaces.contains_key(g[2]) {
                                b_ws = true;
                                a_ws = g[2].to_string();
                                b_db = false;
                                b_tb = false;
                                a_db = "".to_string();
                                a_tb = String::new();
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
                                    b_tb = false;
                                    a_tb = String::new();
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
                        _ => {

                        }
                    }
                },
                    "create" => {
                        if g[1].to_lowercase().as_str() == "db"{
                        g[1] = "database"
                    }

                    if g[1].to_lowercase().as_str() == "ws"{
                        g[1] = "workspace"
                    }
                    
                    match g[1].to_lowercase().as_str() {
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
                    }}

                    "delete" =>{
                        match g[1].to_lowercase().as_str() {

                            "table"=>{

                                if b_db && b_ws{

                                    let temp = workspaces.get_mut(&a_ws).unwrap();
                                    let temp2 = temp.database.get_mut(&a_db).unwrap();
                                    
                                        if temp2.table.contains_key(g[2]){
                                            temp2.table.remove(g[2]);
                                            a_tb = "".to_owned();
                                            b_tb = false;
                                        }else{
                                            logger.update(-17, g[2].to_owned())
                                        }

                                }

                            }

                            "database"=>{

                                if b_ws{

                                    let temp = workspaces.get_mut(&a_ws).unwrap();
                                        if temp.database.contains_key(g[2]){
                                            temp.database.remove(g[2]);
                                            a_db = "".to_owned();
                                            b_db = false;
                                        }else{
                                            logger.update(-9999, g[2].to_owned()) //Non Existent DB
                                        }

                                }

                            }

                            
                            "workspace"=>{

                                if workspaces.contains_key(g[2].clone()){

                                    workspaces.remove(g[2]);
                                    b_ws = false;
                                    a_ws = "".to_owned();

                                }else{
                                    logger.update(-9999, String::new()) //Workspace non existent
                                }

                            }

                            _ => {
                                logger.update(-9999,"".to_owned());
                            }
                        }
                    }
                    _ => {}
                },
                4 => {
                    match g[0].to_lowercase().as_str() {
                        //nabeel
                        "groupby"=>{
                            if b_tb{
                                let column_name = &g[1].to_lowercase().to_string();
                                let aggregate_function = &g[2].to_lowercase().to_string();
                                let function_column = &g[3].to_lowercase().to_string();
                                


                                
                                let temp = workspaces.get(&a_ws).unwrap().database.get(&a_db).unwrap().table.get(&a_tb).unwrap();
                                let order = &temp.order;
                                let mut found_column = false;
                                let mut found_function_column = false;

                                for i in order {
                                    if i == column_name {
                                        found_column = true;
                                        break;
                                    }
                                }

                                for i in order {
                                    if i == function_column {
                                        found_function_column = true;
                                        break;
                                    }
                                }

                               //let mut rows: Vec<_> = temp.cells.get(column_name).unwrap().iter().enumerate().collect();



                                if found_column && found_function_column{
                                    let column_values: Vec<String> = temp.cells
                                            .get(column_name)
                                            .unwrap()
                                            .iter()
                                            .map(|v| v.downcast_ref::<String>().map(|s| s.to_string())
                                                .or_else(|| v.downcast_ref::<i32>().map(|i| i.to_string()))
                                                .or_else(|| v.downcast_ref::<f32>().map(|f| f.to_string()))
                                                .or_else(|| v.downcast_ref::<f32>().map(|f| f.to_string()))
                                                .or_else(|| v.downcast_ref::<bool>().map(|b| b.to_string()))
                                                .or_else(|| v.downcast_ref::<char>().map(|c: &char| c.to_string()))
                                                .unwrap_or_else(|| format!("{:?}", v))
                                            ).collect();
                                    
                                    
                                    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
                                    for (index, value) in column_values.iter().enumerate() {
                                        groups.entry(value.clone()).or_insert_with(Vec::new).push(index);
                                    }

                                    //println!("I AM HERE MOOSA");
                                    apply_aggregate_function(&aggregate_function, function_column, temp, &groups);

                                }else{
                                    logger.update(-1006, "".to_owned());
                                }

                                
                            } else {
                                logger.update(-1008, "".to_owned());
                            }
                        }
                        //nabeel
                        "table" => {
                            if g.len() != 4 {
                                logger.update(-1000, g[1].to_string());
                            }
                            else if g[1] == "from" && g[2] == "CSV" {
                                if b_db && b_ws {
                                    let file_name = &g[3];
                                    let mut ws_to_use = workspaces.get_mut(&a_ws).unwrap();
                                    if b_db {
                                        let db_to_use = ws_to_use.database.get_mut(&a_db).unwrap();
                                        import_table_from_csv(db_to_use, file_name);
                                    } else {
                                        logger.update(-9, g[2].to_owned())
                                    }
                                } else {
                                    logger.update(-10, g[2].to_owned());
                                }
                            }
                            
                        }
                        "import" => match g[1].to_lowercase().as_str(){

                            "as" => match g[2].to_lowercase().as_str(){
    
                                "xml" =>{
    
                                    if b_ws && b_db{
    
                                        let mut lines = Vec::new();
                                        let f = read_to_string(format!("./{}",&g[3])).unwrap();
    
                                        for i in f.lines(){
                                            lines.push(i)
                                        }
                                        
                                        let mut dtypes = lines[0].to_owned();
                                        dtypes = dtypes.replace("<dtypes = ", "").replace("/>", "");
                                        let datatypes = dtypes.split(":").map(|x| x.to_owned()).collect::<Vec<String>>();
                                        //println!("{:?}", &datatypes);
                                        let tablename = g[3].replace(".xml", "");
                                        let colnames = lines[1].to_owned().replace("<colnames = ", "").replace("/>", "").split(":").map(|x| x.to_owned()).collect::<Vec<String>>();
    
                                        let mut temp = Table{
                                            name: tablename,
                                            order: colnames.clone(),
                                            model:datatypes.clone(),
                                            rows: lines.len() as i32 -4,
                                            cells: HashMap::new(),
                                            relations:Vec::new()
                                        };
    
                                        //println!("{:?}", &temp);
    
                                        for i in colnames{
                                            temp.cells.insert(i, Vec::new());
                                        }
    
                                        for i in 3..lines.len()-1{
    
                                            let line = lines[i].to_owned().replace("<Record ", "").replace("/>","").trim().to_owned();
                                            //println!("{}", &line);
                                            let tempcols = line.split(" ").map(|f| f.to_owned()).collect::<Vec<String>>();
                                            let mut k = 0;
    
                                                for j in tempcols{
                                                   // println!("value => {}", &j);
                                                    let val = j.split("=").map(|f| f.to_owned()).collect::<Vec<String>>();
                                                    let mut value = val[1].clone();
                                                    let datatype = temp.model[k].clone().to_lowercase();
                                                    let insertion = temp.cells.get_mut(&val[0]).unwrap();
                                                    k+=1;
    
                                                    //println!("dtype{}",datatype);
                                                    match datatype.as_str(){
                                                        "string" => {
                                                            value = value.replace("\"", "");
                                                            value = value.replace("%20", " ");
                                                            insertion.push(Box::new(value.to_owned()));
                                                        }
                                                        "char" => {
                                                            value = value.replace("\"", "");
                                                            value = value[0..value.len()].to_owned();
                                                            insertion.push(Box::new(value));
                                                        }
                                                        "i32" => {
                                                            insertion.push(Box::new(value.parse::<i32>().unwrap()));
                                                        }
                                                        "integer" => {
                                                            insertion.push(Box::new(value.parse::<i32>().unwrap()));
                                                        }
                                                        "int" => {
                                                            insertion.push(Box::new(value.parse::<i32>().unwrap()));
                                                        }
                                                        "i64" => {
                                                            insertion.push(Box::new(value.parse::<i64>().unwrap()));
                                                        }
                                                        "f32" => {
                                                            insertion.push(Box::new(value.parse::<f32>().unwrap()));
                                                        }
                                                        "float" => {
                                                            insertion.push(Box::new(value.parse::<f64>().unwrap()));
                                                        }
                                                        "boolean" => {
                                                            let test = value.to_lowercase();
                                                            if test == "true"{
                                                                insertion.push(Box::new(true));
                                                            }else{
                                                                insertion.push(Box::new(false));
                                                            }
                                                        }
                    
                                                        _=>{
                                                            //invalid datatype return errors
                                                        }
                                                }
    
                                            }
                                        
                                            //print!("{:?}", &temp);
                                        }
    
    
    
                                        if b_ws{
                                            let mut k = workspaces.get_mut(&a_ws).unwrap();
                                            if b_db{
    
                                                let mut d = k.database.get_mut(&a_db).unwrap(); 
                                                d.table.insert(temp.name.clone(), temp);
    
                                            }else{
                                                logger.update(-9, g[2].to_owned())
                                            }
                                    }
    
                                    }else{
    
                                        if !b_ws{
                                            logger.update(-12, String::new())
                                        }
                                        if !b_db{
                                            logger.update(-13, String::new())
                                        }
    
    
                                    }
    
                                }
    
                                _ =>{
                                    logger.update(-18, g[3].to_owned())
                                }
    
    
                            }
    
                            _=>{
                                logger.update(-09999, "".to_owned())
                            }
                        }
    
                        _=>{
                            logger.update(-09999, "".to_owned())
                        }
                        _ => {
                            // Code for other cases
                        }
                    }
                },
                5=>{
                    
                    match g[0].to_lowercase().as_str(){

                        "create"=>{

                            match g[1].to_lowercase().as_str(){
                                
                                "table"=>{

                                    let x = g[3].split(":").collect::<Vec<&str>>(); //ColNames
                                    let y = g[4].split(":").collect::<Vec<&str>>(); //Col Datatypes

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
                                            let mut data : HashMap<String, Vec<Box<dyn Any + Send>>> = HashMap::new(); //nabeel
                                           
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
