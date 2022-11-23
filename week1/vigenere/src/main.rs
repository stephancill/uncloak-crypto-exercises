// Vigenère cipher
fn main() {
    let plaintext = "attackatdawnattackatdawn".to_uppercase();
    let key = "LEMONLEMONLE";
    let ciphertext = encrypt(plaintext.as_str(), key);
    assert_eq!(decrypt(ciphertext.as_str(), key), plaintext);
}

pub fn encrypt(plaintext: &str, key: &str) -> String {
    let mut ciphertext = String::new();
    let mut key_iter = key.chars().cycle();
    for c in plaintext.chars() {
        let key_char = key_iter.next().unwrap();

        let c_ord = c as u8 - 'A' as u8;
        let key_ord = key_char as u8 - 'A' as u8;
        let cipher_ord = (c_ord + key_ord) % 26;
        ciphertext.push((cipher_ord + 'A' as u8) as char);
    }
    ciphertext
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let mut plaintext = String::new();
    let mut key_iter = key.chars().cycle();
    for c in ciphertext.chars() {
        let key_char = key_iter.next().unwrap();
        println!("{} {}", c, key_char);
        let c_ord = c as u8 - 'A' as u8;
        let key_ord = key_char as u8 - 'A' as u8;
        let plain_ord = (c_ord + 26 - key_ord) % 26;
        plaintext.push((plain_ord + 'A' as u8) as char);
    }
    plaintext
}
