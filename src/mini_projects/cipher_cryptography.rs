pub fn play_it() {
    let key: u8 = 5;

    let message: String = String::from("Hii, I am going to start a war");
    println!("original message -> {}", message);

    let encrypted_message = crypt_message(&message, key, true);
    println!("encrypted message -> {}", encrypted_message);

    let decrypted_message = crypt_message(&encrypted_message, key, false);
    println!("decrypted message -> {}", decrypted_message);
}

fn crypt_message(message: &String, key: u8, to_encrypt: bool) -> String {
    let mut crypted_message: String = String::new();
    for b in message.as_bytes() {
        let crypt_ch = if *b == 32 {
            *b
        } else {
            if to_encrypt {
                *b + key
            } else {
                *b - key
            }
        };
        crypted_message.push(crypt_ch as char);
    }

    return crypted_message;
}
