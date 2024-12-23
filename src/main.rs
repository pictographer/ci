fn main() {
    println!("Hello, new world!");
    println!(
        "version from build env: {}",
        option_env!("GITHUB_REF").unwrap_or("GITHUB_REF unset")
    );
}

#[test]
fn test_trivial_example() {
    assert_eq!(true, true);
}
