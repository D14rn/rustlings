fn main() {
    let smol = String::from("smol :c");

    let smol_never_dies = takes_and_gives_smol(smol); // smol gets owned

    println!("we take ownership of smol (it ded): {smol_never_dies}"); // smol is invalid :c

    let smol_reference = &smol_never_dies;
    println!("we now have a reference to smol: {smol_reference}");
    
    let smol_size = get_length(smol_reference);

    println!("we used the reference to avoid taking ownership of smol: {smol_size}");
    println!("we can still use the variable we just made a reference to: {smol_never_dies}");

    // let fword_index = first_word(smol_reference);
    // println!("index of end of first word: {fword_index}");
    let onest_word = first_word(smol_reference);
    println!("the first word in \"{smol_never_dies}\" is: {onest_word}");
}

fn takes_and_gives_smol(smol_string: String) -> String {
    println!("{smol_string}");
    smol_string
}

fn get_length(x: &String) -> usize {
    x.len()
}

fn first_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[0..i]; // [..i] works too
        }
    }
    &sentence[..]
}

// fn first_word(sentence: &String) -> usize {
    // let bytes = sentence.as_bytes();
// 
    // for (i, &item) in bytes.iter().enumerate() {
        // if item == b' ' {
            // return i;
        // }
    // }
// 
    // sentence.len()
// }


