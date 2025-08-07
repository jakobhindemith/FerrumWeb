# 🕸️ webcrawler_links

A recursive **Rust** web crawler that visits websites, extracts links, and stores them in a **SQLite database**. Each link is saved with its **parent URL** and **depth level**, allowing a structured hierarchy of web connections.

## 🔍 Features

- 🌐 Recursively crawls websites up to a configurable depth  
- 🔗 Extracts all HTML links (`<a href="...">`)  
- 🧭 Stores:
  - the discovered URL,
  - its parent URL,
  - the crawl depth level,
  - and a unique ID
- 🗃️ Persists data in a SQLite database (table: `link`)
- 🧹 Filters duplicate links (optional aggregation)
- ⏳ Optional delay between requests to avoid rate-limiting

## 🛠️ Technologies

- [Rust](https://www.rust-lang.org/)
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

When the program starts, it prompts for a URL and begins crawling from that page. All discovered links are stored recursively in a database. The resulting structure is useful for analyzing site architectures, detecting broken links, or conducting SEO audits.
🧪 Example SQL Queries

  Count of distinct URLs:
  ```sql
  SELECT COUNT(DISTINCT URL) FROM link;
  ```
  Grouped links by frequency:
  ```sql
    SELECT URL, COUNT(*) AS count FROM link GROUP BY URL ORDER BY count DESC;
  ```

## ⚠️ Disclaimer

This tool is intended for educational and research purposes only. Please respect website robots.txt policies and use responsibly.
