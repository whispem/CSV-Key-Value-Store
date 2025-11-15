# CSV Key–Value Store 🦀

A small, exploratory key–value store implemented in Rust.  
This project is part of my ongoing practice around system fundamentals — especially how simple storage engines behave when durability, state transitions, and file-backed formats come into play.  
The goal is to keep the surface intentionally small so each change is conceptually clear.

---

## 📦 What This Is

A minimal CLI-based engine using a CSV file (data.csv) as its persistence layer.  
The design is deliberately straightforward to make behavior observable:

- append + rewrite patterns  
- state restoration on startup  
- trade-offs of naive persistence  
- the beginnings of indexing logic  

This aligns with my broader focus on building small, transparent systems with explicit boundaries.

---

## 🚀 Commands

set <key> <value> — add or update a pair  
get <key> — return a value or “Key not found”  
delete <key> — remove a pair  
list — print all stored pairs  
quit / exit — leave the program  

**Example session:**  
set user Emilie  
set job dev  
get job  
dev  
list  
user,Emilie  
job,dev  
delete user  
get user  
Key not found  
quit  

---

## ⚙️ Installation & Running

git clone https://github.com/whispem/CSV-Key-Value-Store  
cd CSV-Key-Value-Store  
cargo run  

data.csv is created automatically when needed.

---

## 📈 Current Status

Part of a broader learning effort around Rust and small system design.  
Each feature is added with the intention of clarifying how persistent state behaves.

Implemented so far:  
- CSV-backed storage  
- state parsing + restoration  
- set / get / delete logic  
- list functionality  
- clear error pathways  
- simple module layout  

This complements my work on mini-kvstore by focusing specifically on persistence and I/O behavior.

---

## 🧭 Learning Roadmap

Upcoming areas I’m exploring:  
- more robust parsing + validation  
- a small indexing layer to avoid full-file scans  
- write-ahead logging (WAL)  
- compaction strategies  
- early LSM-inspired flow  
- clearer module separation  

Incremental by design — clarity first, complexity only when justified.

---

## 🦀 Why Rust?

This project helps me deepen:  
- ownership and borrowing in the context of file-backed state  
- structured error handling  
- designing micro-systems with explicit lifecycles  
- separating computation from I/O  
- reasoning about durability in safe, transparent ways  

Rust makes these patterns very explicit, which is ideal at this stage.

---

## 📚 Resources

- Rust documentation  
- writing on WAL, LSM trees, indexing, and durability  
- blog posts on small-engine architecture  
- database internals notes  
- general material on simple, correct system design  

Updated as I learn.

---

## 🗒️ Notes

This is an exploratory learning repository — intentionally minimal, intentionally structured.  
The code evolves as my understanding of Rust and storage systems grows.  
Suggestions and reading recommendations are always welcome. 🙏

---

Built while studying Rust and persistence fundamentals — 2025 🦀