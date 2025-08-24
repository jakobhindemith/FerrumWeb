# 🕸️ webcrawler_links

A recursive **Rust** web crawler that visits websites, extracts links, and stores them in a **SQLite database**.

## 🔍 Features

-  Extracts all HTML links (`<a href="...">`)  
-  Stores:
  - the discovered URL
  - and a unique ID
-  Persists data in a SQLite database (table: `link`)
-  Filters duplicate links (optional aggregation)
-  Optional delay between requests to avoid rate-limiting

## Features future
-  Recursively crawls websites up to a configurable depth  
-  Stores:
  - its parent URL
  - the crawl depth level
- Each link is saved with its **parent URL** and **depth level**, allowing a structured hierarchy of web connections.

## 🛠️ Technologies

- [`reqwest`](https://docs.rs/reqwest/) – HTTP client  
- [`scraper`](https://docs.rs/scraper/) – HTML parser  
- [`rusqlite`](https://docs.rs/rusqlite/) – SQLite database integration

## 📦 Installation

```bash
git clone https://github.com/your-username/webcrawler_links.git
cd webcrawler_links
cargo run
```

## 💡Usage

When the program starts, it prompts for a URL and begins crawling from that page. All discovered links are stored recursively in a database. The resulting structure is useful for analyzing site architectures or detecting broken links.

Example SQL Queries:

  Count of distinct URLs:
  ```sql
  SELECT COUNT(DISTINCT URL) FROM link;
  ```
  Grouped links by frequency:
  ```sql
    SELECT URL, COUNT(*) AS count FROM link GROUP BY URL ORDER BY count DESC;
  ```
