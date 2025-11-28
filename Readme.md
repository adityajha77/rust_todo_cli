step 1
cargo add serde --features derive
cargo add serde_json

We will create a function called save_todos. This function needs to do two things:

Open/Create a file (File I/O).

Serialize the vector into JSON and write it to that file.
