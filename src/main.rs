use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Aree Buddy execute the file like cargo run encrypt/decrypt folder-path");
        return;
    }

    println!("{:#?}", args);
}
