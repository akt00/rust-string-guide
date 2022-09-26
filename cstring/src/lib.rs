use std::convert::From;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Index, IndexMut, Range};
use std::iter::{IntoIterator, Peekable};
use std::str;
use std::cmp::{max, PartialEq, PartialOrd, Ordering};
use std::vec::IntoIter;


pub struct CString {
    chars: Vec<u8>
}

impl CString {
    pub fn from_utf8(v: Vec<u8>) -> Self {
        Self { chars: v }
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
    pub fn clone(&self) -> Self {
        Self { chars: self.chars.clone() }
    }
    pub fn to_string(&self) -> String {
        let mut tmp: String = String::new();
        for i in 0..self.len() { tmp += &(self.chars[i] as char).to_string() }
        tmp
    }
    // works like strtol in c. takes in a peekable iterator and moves it to the end of the integer
    pub fn strtol(iter: &mut Peekable<IntoIter<u8>>) -> Option<i32> {
        let mut s = String::new();
        let unary = iter.peek();
        match unary {
            Some(i) => {
                if i.is_ascii_digit() { s += &(*i as char).to_string(); }
                else if *i as char == '+' || *i as char == '-' {
                    s += &(*i as char).to_string();
                    let mut clone = iter.clone();
                    clone.next();
                    if let Some(j) = clone.peek() { if !j.is_ascii_digit() { return None } } 
                    else { return None }
                } else { return None }
            },
            None => return None
        }

        loop {
            iter.next();
            let tmp = iter.peek();
            match tmp {
                Some(i) => {
                    if i.is_ascii_digit() { s += &(*i as char).to_string() } 
                    else { break }
                },
                None => break
            }
        }
        
        if let Ok(i) = s.parse::<i32>() { Some(i) } 
        else { None }
    }

    // works like strtok in C. returns a list of strings separated by delim
    pub fn strtok(txt: Self, delim: char) -> Vec<Self> {
        let mut token = Vec::new();
        let mut tmp = Vec::new();
        for i in txt.chars {
            if i != delim as u8 {
                tmp.push(i)
            }
            else {
                token.push(Self::from_utf8(tmp));
                tmp = Vec::new()
            }
        }
        token.push(Self::from_utf8(tmp));
        token
    }
}

// return 0 if eq, -1 if s1 < s2, 1 if s1 > s2
fn cstring_cmp(s1: &Vec<u8>, s2: &Vec<u8>) -> i32 {
    let mut s1_val = Vec::new();
    let mut s2_val = Vec::new();
    let max_len = max(s1.len(), s2.len());
    for i in 0..max_len {
        if i >= s1.len() {
            s1_val.push(false);
            s2_val.push(true) 
        }
        else if i >= s2.len() {
            s1_val.push(true);
            s2_val.push(false)
        }
        else {
            if s1[i] == s2[i] {
                s1_val.push(true);
                s2_val.push(true)
            } else if s1[i] < s2[i] {
                s1_val.push(false);
                s2_val.push(true)
            }
            else {
                s1_val.push(true);
                s2_val.push(false)
            }
        }
    }
    
    for i in 0..max_len {
        if s1_val[i] == s2_val[i] { continue }
        else if s1_val[i] < s2_val[i] { return -1 }
        else { return 1 }
    }
    0
}


// __ std trait implementatinos ___

impl PartialEq for CString {
    fn eq(&self, other: &Self) -> bool {
        if cstring_cmp(&self.chars, &other.chars) == 0 { return true }
        false
    }

    fn ne(&self, other: &Self) -> bool {
        if cstring_cmp(&self.chars, &other.chars) != 0 { return true }
        false
    }
}

impl PartialOrd for CString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = cstring_cmp(&self.chars, &other.chars);
        match res {
            -1 => Some(Ordering::Less),
            0 => Some(Ordering::Equal),
            1 => Some(Ordering::Greater),
            _ => None
        }
    }
    fn lt(&self, other: &Self) -> bool {
        if cstring_cmp(&self.chars, &other.chars) == -1 { return true }
        false
    }
    fn le(&self, other: &Self) -> bool {
        if cstring_cmp(&self.chars, &other.chars) != 1 { return true }
        false
    }
    fn gt(&self, other: &Self) -> bool {
        if cstring_cmp(&self.chars, &other.chars) == 1 { return true }
        false
    }
    fn ge(&self, other: &Self) -> bool {
        if cstring_cmp(&self.chars, &other.chars) != -1 { return true }
        false
    }
}

impl IntoIterator for CString {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.chars.into_iter()
    }
}

impl From<&str> for CString {
    fn from(ch: &str) -> Self {
        let mut new_str = Self { chars: Vec::new() };
        for i in ch.chars() {
            if i.is_ascii() {
                let tmp = i as u8;
                new_str.chars.push(tmp);
            } else {
                panic!("not an ascii!");
            }
        }
        new_str
    }
}

impl From<&String> for CString {
    fn from(ch: &String) -> Self {
        let mut new_str = Self { chars: Vec::new() };
        for i in ch.chars() {
            if i.is_ascii() {
                let tmp = i as u8;
                new_str.chars.push(tmp);
            } else {
                panic!("not an ascii!");
            }
        }
        new_str
    }
}

impl Add<&str> for CString {
    type Output = Self;
    fn add(self, rhs: &str) -> Self::Output {
        let mut new_str = self.chars.clone();
        for i in rhs.as_bytes() {
            new_str.push(*i);
        }
        Self { chars: new_str }
    }
}

impl AddAssign<&str> for CString {
    fn add_assign(&mut self, rhs: &str) {
        for i in rhs.as_bytes() {
            if i.is_ascii() {
                self.chars.push(*i)
            } else {
                panic!("Error: non ascii character")
            }
        }
    }
}

impl AddAssign<Self> for CString {
    fn add_assign(&mut self, rhs: Self) {
        self.chars.extend(rhs.chars)
    }
}

impl Index<usize> for CString {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl Index<Range<usize>> for CString {
    type Output = str;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        let slice = &self.chars[index];
        str::from_utf8(slice).unwrap()
    }
}

impl IndexMut<usize> for CString {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chars[index]
    }
}

impl Display for CString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String = String::new();
        for i in &self.chars {
            let letter = *i as char;
            let letter_str = String::from(letter);
            text += &letter_str
        }
        write!(f, "{text}")
    }
}




// ____ unit test ____

#[test]
fn test_cstring_cmp() {
    let s1 = CString::from("Glow");
    let s2 = CString::from("Glee");
    assert_eq!(cstring_cmp(&s1.chars, &s2.chars), 1);

    
    let s3 = CString::from("Z");
    let s4 = CString::from("A");
    assert_eq!(cstring_cmp(&s3.chars, &s4.chars), 1);

    
    let s5 = CString::from("Bee");
    let s6 = CString::from("Be");
    assert_eq!(cstring_cmp(&s5.chars, &s6.chars), 1);

    
    let s6 = CString::from("ABC");
    let s7 = CString::from("ABC");
    assert_eq!(cstring_cmp(&s6.chars, &s7.chars), 0);


    
    let s6 = CString::from("Hello");
    let s7 = CString::from("hello");
    assert_eq!(cstring_cmp(&s6.chars, &s7.chars), -1);
}
