# CSV Key-Value Store (Rust)

A small command-line key-value store in Rust.  
It uses a basic CSV file (named `data.csv` in the project folder) to store pairs.

Commands available:

- `set <key> <value>`  — add or update a key-value pair
- `get <key>`     — display a value, or "Key not found"
- `delete <key>`    — remove a key
- `list`         — show all saved pairs
- `quit` or `exit`   — leave the program

Example usage:

```
> set user Emilie
> set job dev
> get job
dev
> list
user,Emilie
job,dev
> delete user
> get user
Key not found
> quit
```

No extra crates, the code is just for learning.  
You can change/improve things as you want — it is a base for experiments.

---
