# CSV Key-Value Store 🦀

**A minimal, educational CSV-based key-value storage engine built in Rust**

[![Rust Version](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Features](#-features) •
[Quick Start](#-quick-start) •
[Usage](#-usage) •
[Learning Goals](#-learning-goals) •
[Roadmap](#-roadmap)

---

## 📚 About

CSV Key-Value Store is a beginner-friendly exploration of persistent storage fundamentals in Rust. 
Built without external crates, this project demonstrates how simple storage layers work by persisting key-value pairs in a minimal CSV format.

This isn't production software—it's a transparent, intentionally simple learning tool designed to understand:
- File-backed persistence
- Read/write cycles
- Trade-offs between append-only vs rewrite models
- Building foundational "real systems" from scratch

> 💡 Perfect for Rust learners who want to understand storage engines from first principles.

---

## ✨ Features

### Core Storage
- 📝 **CSV-based persistence** - Simple, human-readable storage format
- 🔄 **Full CRUD operations** - Set, get, delete, and list keys
- 💾 **Automatic file creation** - `data.csv` created on first run
- 🎯 **Zero dependencies** - Pure Rust implementation
- 🧪 **Interactive CLI** - REPL for testing and exploration

### Learning-Focused Design
- 📖 **Clear, commented code** - Understand every line
- 🎨 **Minimal abstractions** - See exactly what's happening
- 🔍 **Transparent behavior** - No magic, just fundamentals
- 🛠️ **Easy to modify** - Perfect for experimentation

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.75+** - [Install Rust](https://rustup.rs/)

### Installation

```bash
# Clone the repository
git clone https://github.com/whispem/CSV-Key-Value-Store
cd CSV-Key-Value-Store

# Run the project
cargo run
```

### Example Session

```bash
> set user Emilie
Set 'user'

> set job dev
Set 'job'

> get job
dev

> list
user,Emilie
job,dev

> delete user
Deleted 'user'

> get user
Key not found

> quit
Exiting.
```

---

## 💻 Usage

### Available Commands

- `set <key> <value>` - Store or update a key-value pair
- `get <key>` - Retrieve the value for a key
- `delete <key>` - Remove a key-value pair
- `list` - Display all stored key-value pairs
- `quit` / `exit` - Close the application

### Data Persistence

All data is stored in `data.csv` in the project directory:
```csv
user,Emilie
job,dev
location,Marseille
```

The file is automatically created on first run and persists between sessions.

---

## 🏗️ Architecture

### Simple Design

```
┌─────────────────────────┐
│     CLI Interface       │
│   (User Commands)       │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│   In-Memory HashMap     │
│   (Current State)       │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│      data.csv           │
│   (Persistent Store)    │
└─────────────────────────┘
```

### How It Works

**Write Path:**
1. User enters `set key value`
2. Load all entries from CSV into memory
3. Update/add the new key-value pair
4. Write entire dataset back to CSV (overwrites file)

**Read Path:**
1. User enters `get key`
2. Load all entries from CSV
3. Search for key and return value

**Current Limitations:**
- Full file rewrite on every operation
- No indexing (linear search)
- No concurrent access handling
- CSV parsing is basic (no escaping for commas/quotes)

---

## 📚 Learning Goals

This project explores:

### Storage Fundamentals
- **File I/O operations** - Reading from and writing to disk
- **Data serialization** - Converting structures to CSV format
- **Persistence patterns** - Maintaining state between runs

### Rust Concepts
- **Ownership & borrowing** - Working with file handles
- **Error handling** - Using `Result` and `Option` types
- **Collections** - HashMap operations and management
- **String manipulation** - Parsing and formatting data

### Trade-offs
- **Simplicity vs Performance** - Understand the cost of full rewrites
- **Memory vs Disk** - When to load data into memory
- **Flexibility vs Constraints** - CSV's limitations as a storage format

---

## 🗺️ Roadmap

### Current Status ✅
- [x] Basic CSV read/write operations
- [x] Interactive CLI with REPL
- [x] Full CRUD functionality
- [x] Error handling for common cases
- [x] Clean code with comments

### Planned Improvements 📋

**Short-term:**
- [ ] Improved CSV parsing (handle commas in values, quotes)
- [ ] Add command history in CLI
- [ ] Better error messages
- [ ] Input validation
- [ ] Unit tests

**Long-term (Next Projects):**
- [ ] Append-only log (avoid full rewrites)
- [ ] Simple indexing layer
- [ ] Binary format instead of CSV
- [ ] Write-ahead logging (WAL)
- [ ] Concurrent access handling

> 💡 Many of these improvements are already implemented in my [mini-kvstore-v2](https://github.com/whispem/mini-kvstore-v2) project!

---

## 🤔 Design Decisions

### Why CSV?

**Pros:**
- Human-readable format
- Easy to inspect and debug
- No parsing library needed
- Simple to implement

**Cons:**
- Poor performance (full file rewrites)
- Limited data type support
- No efficient indexing
- Fragile parsing

**Learning Value:** Understanding these trade-offs is the point! CSV is perfect for learning the basics before moving to more sophisticated formats.

### Why Full Rewrites?

Currently, every operation rewrites the entire file. This is:
- **Simple to implement** - Easy to understand
- **Correct** - No corruption from partial writes
- **Inefficient** - Slow for large datasets

This intentional simplicity makes the code clear while highlighting why real databases use append-only logs and compaction.

---

## 🦀 Why Rust?

This project uses Rust to learn:
- **Memory safety** - No segfaults while learning file I/O
- **Explicit error handling** - Understanding what can fail
- **Ownership model** - Managing file handles safely
- **Zero-cost abstractions** - Performance without complexity
- **Strong type system** - Catching bugs at compile time

---

## 🤝 Contributing

This is primarily a learning project, but suggestions are welcome!

### Ways to Contribute

- 🐛 **Report issues** - Found a bug? Let me know!
- 💡 **Share ideas** - Suggestions for improvements
- 📖 **Improve docs** - Clarifications or examples
- 🎓 **Learning resources** - Know great articles or books?

### If You're Learning Too

Feel free to fork this project and:
- Add your own features
- Experiment with different storage formats
- Implement the "Planned Improvements"
- Share what you learned!

---

## 📚 Learning Resources

### Storage Systems
- [Database Internals](https://www.databass.dev/) by Alex Petrov
- [Designing Data-Intensive Applications](https://dataintensive.net/) by Martin Kleppmann
- [Build Your Own Database](https://build-your-own.org/database/) - Great hands-on guide

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises

### Next Steps
After mastering the basics here, check out:
- [mini-kvstore-v2](https://github.com/whispem/mini-kvstore-v2) - My production-ready KV store
- [tiny-log-kv](https://github.com/whispem/tiny-log-kv) - Append-only log implementation
- [sled](https://github.com/spacejam/sled) - Real-world embedded database

---

## 📜 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

## 👤 Author

**Em' ([@whispem](https://github.com/whispem))**

From literature background to building storage engines. This is one of my first explorations into database fundamentals—learning by building, one project at a time.

> *"Start simple, understand deeply, then grow."*

---

## 📬 Contact

- 🐛 **Issues:** [GitHub Issues](https://github.com/whispem/CSV-Key-Value-Store/issues)
- 💬 **Suggestions:** Open a discussion!
- 📧 **Email:** contact.whispem@gmail.com

---

## 🌟 Part of My Learning Journey

This project is part of my progression in Rust and storage systems:

1. **CSV-KV Store** ← You are here (Basics)
2. [Tiny Log-KV](https://github.com/whispem/tiny-log-kv) (Append-only logs)
3. [Mini KVStore](https://github.com/whispem/mini-kvstore) (In-memory foundations)
4. [Mini KVStore v2](https://github.com/whispem/mini-kvstore-v2) (Production-ready)

Each project builds on lessons from the previous one!

---

**Built with ❤️ and curiosity in Rust**

[⬆ Back to Top](#csv-key-value-store-)
