#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

fn is_strong(pw: String) -> bool {
    pw.len() > 5
}

fn is_strong_asref<T: AsRef<str>>(pw: T) -> bool {
    pw.as_ref().len() > 5
}

fn is_strong_into_string<T: Into<String>>(pw: T) -> bool {
    pw.into().len() > 5
}

fn is_strong_into_vec<T: Into<Vec<u8>>> (s: T) -> bool {
    let bytes = b"hello".to_vec();
    let convert_s: Vec<u8> = s.into();

   if assert_eq!(bytes, convert_s) == () {
       return true;
   }
   false
}

#[test]
fn test_original() {
    let pw1 = String::from("justok");
    assert!(is_strong(pw1));

    // let pw2: &str = "justok";
    // assert!(is_strong(pw2)); // error
}

#[test]
fn test_asref() {
    let pw1 = String::from("justok");
    assert!(is_strong_asref(pw1));

    let pw2: &str = "justok";
    assert!(is_strong_asref(pw2));
}

#[test]
fn test_into_string() {
    let pw1 = String::from("justok");
    assert!(is_strong_into_string(pw1));

    let pw2: &str = "justok";
    assert!(is_strong_into_string(pw2));
}

#[test]
fn test_into_vec() {
    let pw1 = "hello";
    assert!(is_strong_into_vec(pw1));

    let pw2 = "hello".to_string();
    assert!(is_strong_into_vec(pw2));

    let pw3 = String::from("hello");
    assert!(is_strong_into_vec(pw3));
}