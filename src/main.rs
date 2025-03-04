use symmetric_pre::{decrypt, encrypt, re_encrypt, Key, ReEncryptionKey};

fn main() {
    let message = b"les sanglots longs des violons !";
    println!("message: {:?}", message);
    let key1 = Key::generate();
    let encrypted = encrypt(message, &key1);
    println!("encrypted: {:?}", encrypted);
    let decrypted = decrypt(&encrypted, &key1);
    println!("decrypted: {:?}", str::from_utf8(&decrypted).unwrap());

    let key2 = Key::generate();
    let re_encryption_key = ReEncryptionKey::generate(&key1, &key2);
    let re_encrypted = re_encrypt(&encrypted, &re_encryption_key);
    println!("re_encrypted: {:?}", re_encrypted);
    let re_decrypted = decrypt(&re_encrypted, &key2);
    println!("re_decrypted: {:?}", str::from_utf8(&re_decrypted).unwrap());
}
