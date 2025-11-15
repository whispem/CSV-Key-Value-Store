# CSV Key–Value Store 🦀

A small, exploratory command-line key–value store written in Rust.  
This project focuses on understanding how simple storage layers behave by persisting key–value pairs in a minimal CSV file (`data.csv`) without external crates.  
The goal: build clear, tiny systems while learning the fundamentals of persistence and file-backed state.

---

## 🧩 What This Is

A tiny CLI-based key–value engine designed to explore:

- how simple storage works under the hood  
- naive persistence through CSV  
- read/write cycles  
- trade-offs of append-only vs rewrite models  
- shaping small scripts into early-stage “real systems”

This repo is intentionally simple, intentionally transparent, and built for learning.

---

## ⚙️ Installation & Running

Clone the repository:

```bash
git clone https://github.com/whispem/CSV-Key-Value-Store
cd CSV-Key-Value-Store
cargo run
```

`data.csv` will be created automatically if it doesn’t exist.

---

## 💬 Example Session

```
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
```

---

## 📌 Current Status

Part of a broader Rust learning journey (2025).  
Current phase: improving clarity, persistence behavior, and basic storage patterns.

Implemented so far:

- CSV-based persistence  
- parsing and loading existing data  
- `set` / `get` / `delete`  
- `list`  
- interactive CLI  
- basic error handling  

Each feature is intentionally small so the behavior stays fully understandable.

---

## 📈 Learning Roadmap

Planned improvements:

- more robust CSV parsing  
- small indexing layer (avoid full scans)  
- write-ahead logging (WAL)  
- handling partial writes / corruption  
- compaction experiments  
- early LSM-inspired ideas  
- cleaner module structure  

Incremental by design — clarity first.

---

## 🦀 Why Rust?

This project is a way to practice:

- ownership & borrowing with file-backed data  
- clean error handling  
- shaping tiny, explicit system components  
- separating concerns in small Rust programs  
- understanding how persistence interacts with memory safety and I/O  

Perfect playground for learning storage fundamentals.

---

## 📚 Resources

- Rust documentation  
- blog posts about storage engines  
- articles on WAL, LSM trees, durability  
- writing on small, clear system design  
- deep dives into database internals  

Updated as I learn.

---

## 🗒️ Notes

This is a beginner learning project — intentionally minimal and exploratory.  
The code evolves as I better understand Rust, storage, and system design.  
Suggestions or reading recommendations are always welcome. 🙏

---

Built while exploring Rust and file-backed storage — 2025 🦀