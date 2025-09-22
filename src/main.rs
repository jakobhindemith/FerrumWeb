mod crawler;
mod db;
use std::path::Path;
use std::io;
use std::fs::File;
use std::time:: Instant;

//read url's
fn main() {

    //file .txt -> rename
    let path = Path::new("C:\\Rust\\webcrawler_links\\link_results.txt");
    let mut file = File::create(path).expect("error create file");
    
    //commandline input
    let mut url_input = String::new();
    let mut depth_str = String::new();

    println!("
'||''''| '||''''|  '||''''|  '||''|.  '||'  '|' '||    ||'                     '||      
 ||  .    ||  .     ||  .     ||   ||  '|.  .'   |||  |||  ... ... ...   ....   || ...  
 ||''|    ||''|     ||''|     ||''|'    ||  |    |'|..'||   ||  ||  |  .|...||  ||'  || 
 ||       ||        ||        ||   |.    |||     | '|' ||    ||| |||   ||       ||    | 
.||.     .||.....| .||.....| .||.  '|'    |     .|. | .||.    |   |     '|...'  '|...'
");

    println!("Enter link: ");
    io::stdin().read_line(&mut url_input).expect("url_input error");
    let url_input = url_input.trim().to_string();
    
    println!("Enter depth to search: ");
    io::stdin().read_line(&mut depth_str).expect( "depth input error");
    let depth_max: usize = depth_str.trim().parse().expect("musst be a non negative integer");
    
    //db Connection -> init
    let conn = db::init_db().expect("DB Fail");
    //start with depth 0
    let depth = 0;
    //start with parent id = 1
    let parent_id = 0;

    //insert input link -> db
    db::insert_link(&conn, &url_input, depth, parent_id).expect("Failed to insert url_input link");

     //Timer for Runtime
    let start = Instant::now();

    //search links
    match crawler::seaker(&conn, url_input, depth, &mut file, depth_max, parent_id) {
        Ok(_) => println!("Scraping successful!"),
        Err(e) => eprintln!("Error seaking : {}", e),
    }

    //Timer ends
    let elapsed = start.elapsed().as_secs_f32();
    println!("Runtime:  {} sec", elapsed);
}