use evil_id::EvilId;

fn main() {
    let mut errors: u64 = 0;

    for i in 1..=1000000 {
        let id = EvilId::generate();

        let string = id.get();

        let rebuilt_id = EvilId::new_from(string.clone());

        match rebuilt_id {
            Err(_) => {
                errors += 1;
                println!("Error rebuilding id {} on index {i}.", string.clone())
            }
            Ok(rebuilt_id) => {
                if id != rebuilt_id {
                    println!("Rebuilt ID from {string} doesn't match original on index {i}.");
                }
            }
        }
    }

    if errors == 0 {
        println!("Test went with no errors whatsoever.");
    } else {
        println!("{errors} errors found!");
    }
}
