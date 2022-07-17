use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex_literal::hex;

pub fn _1_anatomy_fun() {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(b"my secret and secure key").expect("Hmac can take key of any size");
    mac.update(b"input message");
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let expected = hex!("97d2a569059bbcd8ead4444ff99071f4c01d005bcefe0d3567e1be628e5fdcd9");
    assert_eq!(code_bytes[..], expected[..]);
}

pub fn _2_verify_slice_fun(){
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(b"my secret and secure key").expect("Hmac can take key of any size");
    mac.update(b"input message");
    let code_bytes = hex!("97d2a569059bbcd8ead4444ff99071f4c01d005bcefe0d3567e1be628e5fdcd9");
    mac.verify_slice(&code_bytes[..]).unwrap();
}