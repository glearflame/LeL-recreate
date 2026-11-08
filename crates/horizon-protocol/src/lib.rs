//! Stub para horizon-protocol (baseado em horizon-rust/horizon MIT).

pub mod rsa {
    //! Criptografia RSA (stub).

    pub struct RsaKey;
    pub struct RsaPrivateKey;
    pub struct RsaPublicKey;

    impl RsaKey {
        pub fn new() -> Self { Self }
    }

    impl RsaPrivateKey {
        pub fn new() -> Self { Self }
    }

    impl RsaPublicKey {
        pub fn new() -> Self { Self }
    }
}
