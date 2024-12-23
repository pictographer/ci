use sha256::try_digest;
use std::env::current_exe;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("Hello, new world!");
    let checksum = try_digest(current_exe()?)?;
    println!("SHA256: {checksum}");
    println!(
        "GITHUB_REF: {}",
        option_env!("GITHUB_REF").unwrap_or("unset")
    );
    Ok(())
}

#[test]
fn test_trivial_example() {
    assert_eq!(true, true);
}
