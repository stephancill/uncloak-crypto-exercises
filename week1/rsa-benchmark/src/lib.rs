// TODO: Improvements
// Refactor key generation
// Run both benchmarks simultaneously

pub fn rsa_encrypt(data: &[u8]) {
    let _enc = rsa_lib_2::rsa_encrypt(data);
}

mod rsa_lib_1 {
    use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};

    pub fn rsa_encrypt(data: &[u8]) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
    
        // Encrypt
        let enc_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..]).expect("failed to encrypt");
        return enc_data;
    }
}

mod rsa_lib_2 {
    use openssl::rsa::{Rsa, Padding};

    pub fn rsa_encrypt(data: &[u8]) -> Vec<u8> {
        let rsa = Rsa::generate(2048).unwrap();
        let mut buf = vec![0; rsa.size() as usize];
        let _encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1).unwrap();
        return buf;
    }
}
