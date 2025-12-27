use text_to_ascii_art::to_art;

fn main() {
    println!("Hello World!");

    // lets also test using a crate 🦀
    match to_art("Hello World!".to_string(), "standard", 2, 2, 2) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }
}
