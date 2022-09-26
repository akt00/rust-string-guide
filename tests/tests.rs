use std::cmp::Ordering;
use cstring::CString;


#[test]
fn test_init() {
    let s = CString::from("hello");
    let tmp = "hello";
    let mut iter = tmp.chars().into_iter();

    for i in 0..s.len() {
        assert_eq!(s[i], iter.next().unwrap() as u8);
    }
}

#[test]
fn test_len() {
    let s1 = CString::from("");
    assert!(s1.len() == 0);
    let s2 = CString::from("hello world");
    assert!(s2.len() == 11);
}

#[test]
fn test_to_string() {
    let s1 = CString::from("hello world");
    let tmp = s1.to_string();
    let s2 = String::from("hello world");
    assert!(tmp == s2);
}

#[test]
fn test_strtol() {
    let s1 = CString::from("0");
    let mut iter1 = s1.into_iter().peekable();
    let res1 = CString::strtol(&mut iter1);
    assert!(res1.unwrap() == 0);
    assert!(iter1.next().is_none() == true);

    
    let s2 = CString::from("-2134f3fd");
    let mut iter2 = s2.into_iter().peekable();
    let res2 = CString::strtol(&mut iter2);
    assert!(res2.unwrap() == -2134);
    assert!(iter2.next().unwrap() as char == 'f');

    
    let s3 = CString::from("-f3fd");
    let mut iter3 = s3.into_iter().peekable();
    let res3 = CString::strtol(&mut iter3);
    assert!(res3.is_none() == true);
    assert!(iter3.next().unwrap() as char == '-');
}

#[test]
fn test_strtok() {
    let txt = "this message - shall pass - the unit test";
    let res = CString::strtok(CString::from(txt), '-');
    let mut iter = res.into_iter();
    assert!(iter.next().unwrap() == CString::from("this message "));
    assert!(iter.next().unwrap() == CString::from(" shall pass "));
    assert!(iter.next().unwrap() == CString::from(" the unit test"));
    assert!(iter.next().is_none() == true);
}

#[test]
fn test_partial_eq() {
    let s1 = CString::from("hello");
    let s2 = CString::from("hello");
    let res = s1.partial_cmp(&s2).unwrap();
    assert!(res == Ordering::Equal);
    assert!(s1 == s2);
}

#[test]
fn test_partial_ne() {
    let s1 = CString::from("hello");
    let s2 = CString::from("world");
    assert!(s1 != s2);
}

#[test]
fn test_partial_lt() {
    let s1 = CString::from("Hello");
    let s2 = CString::from("hello");
    let res = s1.partial_cmp(&s2).unwrap();
    assert!(res == Ordering::Less);
    assert!(s1 < s2);
}

#[test]
fn test_partial_le() {
    let s1 = CString::from("Hello");
    let s2 = CString::from("hello");
    let s3 = CString::from("Hello");
    assert!(s1 <= s2);
    assert!(s1 <= s3);
}

#[test]
fn test_partial_gt() {
    let s1 = CString::from("hello");
    let s2 = CString::from("Hello");
    let res = s1.partial_cmp(&s2).unwrap();
    assert!(res == Ordering::Greater);
    assert!(s1 > s2);
}

#[test]
fn test_partial_ge() {
    let s1 = CString::from("hello");
    let s2 = CString::from("Hello");
    let s3 = CString::from("hello");
    assert!(s1 >= s2);
    assert!(s1 >= s3)
}
