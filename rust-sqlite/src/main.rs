use std::result::Result; // Importing the Result type for handling errors
use sqlx::{sqlite::SqliteQueryResult,Sqlite,SqlitePool,migrate::MigrateDatabase};

async fn create_schema(db_url:&str)->Result<SqliteQueryResult,sqlx::Error>{
     // Connect to the SQLite database asynchronously and handle any errors.
let pool=SqlitePool::connect(&db_url).await?;
    // Define the SQL query to enable foreign keys and create the tables if they do not exist.
let qry="PRAGMA foreign_keys= ON;
CREATE TABLE IF NOT EXISTS settings (
    settings_id    INTEGER PRIMARY KEY NOT NULL,
    description    TEXT                NOT NULL,
    created_on     DATETIME DEFAULT    (DATETIME('now','localtime')),
    updated_on     DATETIME DEFAULT    (DATETIME('now','localtime')),
    done           BOOLEAN             NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS project(
    project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    product_name                 TEXT ,
    created_on                   DATETIME DEFAULT (datetime('now','localtime')),
    updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
    img_directory                TEXT NOT NULL,
    out_directory                TEXT NOT NULL,
    status                       TEXT NOT NULL,
    settings_id                  INTEGER  NOT NULL DEFAULT 1,
    FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
);";

// Execute the SQL query and handle any errors.
let result=sqlx::query(&qry).execute(&pool).await;
pool.close().await;  // Close the connection to the database.
return result; // Return the result of the query execution.
}

#[async_std::main] //Define the main function as an asynchronous function using async-std.
async fn main(){
     // Define the database URL.
let db_url=String::from("sqlite://sqlite.db");
    // Check if the database exists, and create it if it does not.
if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
    Sqlite::create_database(&db_url).await.unwrap();  // Create the database.
    match create_schema(&db_url).await{
Ok(_)=>println!("Database created sucessfully"),
Err(e) => panic!("{}",e)  // Panic and print the error if there is one.
    }
}
// Connect to the database and insert a test record into the settings table.
let instances = SqlitePool::connect(&db_url).await.unwrap();
let qry="INSERT INTO settings (description) VALUES(?)";  // Define the insert query with a parameter placeholder.
let result=sqlx::query(&qry).bind("testing").execute(&instances).await;
instances.close().await;  // Close the database connection.
println!("{:?}",result); // Print the result of the insert operation.
}