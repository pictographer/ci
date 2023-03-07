use std::include_str;

fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        option_env!("GITHUB_REF").unwrap_or("GITHUB_REF unset")
    );
    println!("version: {}", include_str!("../version"));
}
