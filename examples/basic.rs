use evil_id::EvilId;

fn main() {
    let id: EvilId = EvilId::generate(); // A new id can be created with the generate function.

    let id_string: String = id.get(); // The stringify function creates a 16 character string representing the id.

    let also_id: EvilId = EvilId::new_from(id_string.clone()).expect("Shouldn't happen."); // We can get the id back from a String. MAke sure to give it a valid id string or it will throw an error.

    let is_eq = id == also_id; // They should be the same.

    println!("ID: {id_string}\nIs eq: {is_eq}");
}
