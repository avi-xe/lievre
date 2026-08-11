use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::RsaPrivateKey;

/// Generated key pair for HTTP signatures
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key_pem: String,
    pub private_key_pem: String,
}

/// Generate an RSA key pair for a new user
pub fn generate_keypair() -> anyhow::Result<KeyPair> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;

    let private_key_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)?
        .to_string();

    let public_key = private_key.to_public_key();
    let public_key_pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)?;

    Ok(KeyPair {
        public_key_pem,
        private_key_pem,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let keypair = generate_keypair().unwrap();
        assert!(!keypair.public_key_pem.is_empty());
        assert!(!keypair.private_key_pem.is_empty());
        assert!(keypair.public_key_pem.contains("BEGIN PUBLIC KEY"));
        assert!(keypair.private_key_pem.contains("BEGIN PRIVATE KEY"));
    }
}
