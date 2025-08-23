mod crawler;
mod db;
use std::io;
use std::fs::File;


//read url's
fn main() {
    //create file to write links to
    let mut file = File::create("C:\\Rust\\webcrawler_links\\link_results.txt").expect("Faild create file");

    //commandline input
    let mut url_input = String::new();
    println!("Enter link: ");
    io::stdin().read_line(&mut url_input).expect("url_input: error");
    let url_input = url_input.trim().to_string();
    
    //db Connection -> init
    let conn = db::init_db().expect("DB Fail");

    //search links
    match crawler::seaker(&conn, url_input, 0, &mut file) {
        Ok(_) => println!("Scraping successful!"),
        Err(e) => eprintln!("Error seaking : {}", e),
    }
}