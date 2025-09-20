# 🕸️ FERRVMweb

This is a recursive web crawler written in **Rust** that visits websites, extracts links, and stores them in a **SQLite database**.

<img width="907" height="763" alt="grafik" src="https://github.com/user-attachments/assets/75ef0ffa-e108-4807-be64-b80fb4049fe1" />

## 🔍 Features

-  Extracts all HTML links (`<a href="...">`)  
Stores:
  - the discovered URL wtih a unique ID and parent ID
  -  Persists data in a SQLite database (table: `link`)
  -  Recursively crawls websites up to a configurable depth  

## Planned future

- Each link is saved with its **parent URL** and **depth level**, allowing to visualize a structured hierarchy of web with the help of [`egui_graphs`] (https://crates.io/crates/egui_graphs/0.9.0)

## 📦 Crates

- [`reqwest`](https://docs.rs/reqwest/) – HTTP client  
- [`scraper`](https://docs.rs/scraper/) – HTML parser  
- [`rusqlite`](https://docs.rs/rusqlite/) – SQLite database integration 

## 💿 Installation

```bash
git clone https://github.com/jakobhindemith/webcrawler_links
cd webcrawler_links
cargo run (if rust is installed: https://www.rust-lang.org/tools/install )
```

## 💡Usage

When the program starts, it asks for a URL and begins crawling from that page. All discovered links are stored recursively in a database. The resulting structure is useful for analyzing site architectures or detecting broken links.

To analyse the DB file simply open a DBMS of your choice.
For Example the **DB Browser for SQLite:**  https://sqlitebrowser.org/

Example SQL Queries:

  Count of distinct URLs:
  ```sql
  SELECT COUNT(DISTINCT URL) FROM link;
  ```
  Grouped links by frequency:
  ```sql
    SELECT URL, COUNT(*) AS count FROM link GROUP BY URL ORDER BY count DESC;
  ```
