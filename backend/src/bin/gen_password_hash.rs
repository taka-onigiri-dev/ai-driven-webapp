use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "Password123";

    match hash(password, DEFAULT_COST) {
        Ok(hashed) => {
            println!("========================================");
            println!("Password Hash Generator");
            println!("========================================");
            println!();
            println!("Password: {}", password);
            println!("Hash:     {}", hashed);
            println!();
            println!("Use this hash in SQL files:");
            println!("'{}'", hashed);
            println!();
        }
        Err(e) => {
            eprintln!("Error generating hash: {}", e);
            std::process::exit(1);
        }
    }
}
