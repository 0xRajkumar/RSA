use std::env;
mod math;
mod rsa;
fn main() {
    let args: Vec<String> = env::args().collect();
    let first = &args[1];
    println!("{first}");
    match first.as_str() {
        "gen" => rsa::gen(),
        "en" => rsa::en_fun(),
        "de" => rsa::de_fun(),
        _ => println!(":-("),
    }
}
