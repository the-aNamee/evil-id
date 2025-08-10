This is a simple crate that I have created because I needed an id and I can't ever make things simple for myself.

`EvilID` is pretty simple.```let id: EvilID = EvilID::generate(); // A new id can be created with the generate function.

let id_string: String = id.get(); // The stringify function creates a 16 character string representing the id.

let also_id: EvilID = EvilID::new_from(id_string.clone()).expect("Shouldn't happen."); // We can get the id back from a String. MAke sure to give it a valid id string or it will throw an error.

let is_eq = id == also_id; // They should be the same.

println!("ID: {id_string}\nIs eq: {is_eq}");```
This code will return something like:```ID: QFTF-PAMI-DGME-KJXA
Is eq: true```

What is evil you might ask? Just look inside of the repo!
