pub fn play_it() {
    println!("Hello strings and their operations!");

    let mut sentence: String = String::new();
    sentence.push('I');
    sentence.push_str(" am Ahmad Raza");

    for word in sentence.split_whitespace() {
        println!("{}", word);
    }

    sentence = sentence.replace("I", "He").replace("am", "is");

    println!("{}", sentence);

    let secret_code = String::from("a s d f w n w n w l q p");

    let mut secret_chars: Vec<char> = secret_code.chars().collect();
    secret_chars.sort();
    secret_chars.dedup(); // deletes consecutive repeatative elements
    secret_chars.reverse();
    secret_chars.pop();
    secret_chars.reverse();

    for s_c in secret_chars {
        println!("{}", s_c);
    }

    let peace: &str = "Everything is fine";
    let peace_2: String = peace.to_string();

    let byte_peace: &[u8] = peace_2.as_bytes();
    let mut itera = peace_2.chars();

    for mut b in byte_peace {
        let temp = *b + 1;
        b = &temp;
        println!("{} --> {}", *b as char, b);
        itera.next();
    }

    let hii = String::from("hii");
    let hello: String = String::from(" how are u");
    let greet: String = hii + &hello;
    println!("{}", greet);
}
