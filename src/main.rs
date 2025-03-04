use symmetric_pre::{decrypt, encrypt, Key};

fn main() {
    let message = b"les sanglots longs des violons !";
    println!("message: {:?}", message);
    let key = Key::generate();
    let encrypted = encrypt(message, &key);
    println!("encrypted: {:?}", encrypted);
    let decrypted = decrypt(&encrypted, &key);
    println!("decrypted: {:?}", decrypted);
}
