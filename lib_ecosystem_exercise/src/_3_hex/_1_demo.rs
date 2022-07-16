use hex::{encode, decode, encode_upper, FromHexError};

pub fn encode_fn() {
    assert_eq!(encode("Hello world!"), "48656c6c6f20776f726c6421");
    assert_eq!(encode(vec![1, 2, 3, 15, 16]), "0102030f10");
}

pub fn decode_fn() {
    assert_eq!(decode("48656c6c6f20776f726c6421"), Ok("Hello world!".to_owned().into_bytes()));
    assert_eq!(decode("123"), Err(FromHexError::OddLength));
    assert!(decode("foo").is_err());
}

pub fn encode_upper_fn() {
    assert_eq!(encode_upper("Hello world!"), "48656C6C6F20776F726C6421");
    assert_eq!(encode_upper(vec![1, 2, 3, 15, 16]), "0102030F10");
}