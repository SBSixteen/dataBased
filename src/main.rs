pub mod dataBased{
    
    use chrono::Utc;

    #[derive(Debug)]
    pub struct Workspace{
        name: String,
        author: String,
        metadata: String,
        database:Vec<Db>
    }

    #[derive(Debug)]    
    pub struct Db{
        name:String,
        table: Vec<Table>
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

        pub fn getDB(&self) -> &Vec<Db>{

            return &self.database;

        }

        fn getDBNames(&self) -> String{

            let g = &self.database;

            let mut x = String::from("[");

            for i in g{

                x.push_str(&i.name);
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

            let x = Db{
                name:x,
                table:Vec::new()
            };

            let _ = &self.database.push(x);

        }

    }
    
    pub fn create_workspace(x:String, y:String) -> Workspace{

        let mut meta = String::from("");

        meta.push_str("Date Created : ");
        meta.push_str(&Utc::now().to_string());

        let mut x = Workspace{

            name: x,
            author: y,
            metadata: meta,
            database: Vec::new()

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
