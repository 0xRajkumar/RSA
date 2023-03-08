use std::env;
mod math;
mod rsa;
fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = &args[1];
    match flag.as_str() {
        "gen" => rsa::gen(),
        "en" => rsa::en_fun(),
        "de" => rsa::de_fun(),
        _ => println!(":-("),
    }
}
