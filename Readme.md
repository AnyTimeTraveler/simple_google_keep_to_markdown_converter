# Simple Google Keep Export to Markdown Converter

## Running

0. [Have Rust installed](https://rustup.rs/)
1. Clone this repo
2. Extract your [Google Takeout](https://takeout.google.com/) Keep Data into this repo
    - The file structure should look like this: `<repo root>/Takeout/Keep/` with many `.json` files inside
3. `cargo run`
4. Next to every json file, should now be a markdown file with the same content. `Bla.json ==> Bla.md`

## Missing features

I only used lists and text notes and built this within 15 minutes.  
This program might break on notes that use attach things like locations etc.  
Feel free to file an issue, and maybe I'll find it fun enough to implement it ;)  
If you file an issue, please attach the error message that you got and the relevant `.json` file that the error mentions.
