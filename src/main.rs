use std::include_str;

fn main() {
    println!("Hello, new world!");
    println!(
        "version from build env: {}",
        option_env!("GITHUB_REF").unwrap_or("GITHUB_REF unset")
    );
    println!("version from file: {}", include_str!("../version"));
}
