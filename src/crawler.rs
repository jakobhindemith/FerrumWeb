use crate::db;
use reqwest;
use rusqlite::Connection;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::Write;

// webscraper
pub fn seaker(conn: &Connection, url_input: String, depth: usize, file: &mut File, depth_max: usize) -> Result<(), Box<dyn Error>> {

    //max depth to search through
    if depth > depth_max {
        return Ok(());
    }
    
    //init client to send request
    let client = reqwest::blocking::Client::new();

    //send request
    let response = client.get(&url_input)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, wie Gecko) Chrome/100.0.4896.127 Safari/537.36")
        .send()?;
    

    //ask status
    if !response.status().is_success() {
        eprintln!("Failed to call: {}, Status: {}", url_input, response.status());
    }
    
    //search for new links in input url -> look for new links in those and so on...
    let body = response.text()?;
    let document = Html::parse_document(&body);
    let selector_link = Selector::parse("a").unwrap();

    println!("links found on {} (Depth: {})", url_input, depth);
    println!("---------------------------------------------------");

    //vector with new links
    let mut new_links = Vec::new();

    //list all links
    for element in document.select(&selector_link){
        if let Some(link) = element.value().attr("href"){
        //owner -> String
        let absolute_link: String = if link.starts_with("http://") || link.starts_with("https://"){
            link.to_string()
        }else{
            continue;
        };

            //print discovert links
            println!("- {} Depth: {}", absolute_link, depth);
            
            //parse absolute_link -> insert_link
            db::insert_link(conn ,&absolute_link, depth)?;

            //write links into file
            file.write_all(absolute_link.as_bytes())?;
            file.write_all(b"\n")?;
            //no doubles
            if !new_links.contains(&absolute_link){
                new_links.push(absolute_link);
             }
        }
    }
    println!("---------------------------------------------------");

    //wait time -> performance
    //thread::sleep(Duration::from_secs(2));

    //Recursive call -> new links -> search trough new links +1 depth
    for link in new_links{
        seaker(conn, link, depth+1, file, depth_max).expect("seaker error");
    }
    Ok(())
}
