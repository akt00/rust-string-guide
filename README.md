# Rust-String-Guide

## Preface

### *Motivation*
Coming from the background of C/C++ and a bit of Python and Javascript, I found the Rust's string type system a little confusing. So I wrote this down to help understand the Rust's string data type.
### *Objectives*
Explains Rust's string data types with examples. I have also made an ASCII string datype which you can use just like the string in C/C++ as a toy project. 
### *Expected Level*
Beginner
### *References*
1. [Rust By Example](https://doc.rust-lang.org/rust-by-example/std/str.html)
2. [Rust-doc String](https://doc.rust-lang.org/std/string/struct.String.html)
3. [Rust-doc str](https://doc.rust-lang.org/std/primitive.str.html)


## Introduction to Rust's string data types

### *Rust's char type*
This is the first caveat when learning Rust's string. Rust's string is UTF-8 by default. A charater in UTF-8 can be either 1, 2, 3 or 4 bytes. Thus, Rust's char is 4 bytes by default. This is different from C/C++'s char data type which is 1 byte by default.

Example
```rb
  println!("size of char = {} bytes", size_of::<char>());
```
Output
```
  size of char = 4 bytes
```

### *String and &str*
In Rust, you often see two types of strings, String and &str. According to the [Rust by Example](https://doc.rust-lang.org/rust-by-example/std/str.html), the difference is 

> A String is stored as a vector of bytes (Vec\<u8\>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated. \
&str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.

This basically means String is a Vec\<u8\> container which contains UTF-8 characters allocated on heap, and &str is a view over a str primitive object. In other words, String owns the object itself while &str references the object.


Bellow, s1 is allocated on heap with String::from method. s2 is a string view over a string literal which is directory embedded in the binary.
```rb
  let s1 = String::from("hello"); // String
  let s2 = "Hello"; // &str
```

Since &str is a view, you cannot modify the value it is referencing unlike String.
```rb
  let mut s1 = String::from("hello"); // String
  let mut s2 = "hello"; // &str

  s1 += " world"; // ok
  s2 += " world"; // error
```

You can also create a view over a String.
```rb
  let s = String::from("hello world");
  let view: &str = &s;
  println!("{view}");
```
Output
```
  hello world
```

### *Indexing*
In C/C++, you can access the characters in a string by indexing like so,
```rb
  std::string str = "pen";
  str[1] = 'a';
  std::cout << str << std::endl;
```
Output
```
  pan
```

However, in Rust, it is not possible to access the elements in a string by indexing because the UTF-8 character is variable length.
```rb
  let mut s = String::from("pen");
  s[1] = 'a'; // error
```
You can access each element through an iterator. However, you cannot randomly index into a character nor change the value because a UTF-8 character in a string is not easy to replace due to variable length.
```rb
  let s = String::from("hello");
  for i in s.chars() {
    print!("{i}");
  }
```
Output
```
hello
```

---
## Examples

### *Conversion*
String to &str
```rb
  let s = String::from("hello world");
  let copy = &s as &str;
```
String to Vec\<u8\>
```rb
  let s = String::from("hello world");
  let copy = s.as_bytes().to_vec();
```
&str to String
```rb
  let s = "hello world";
  let copy = s.to_string();
```
&str to Vec\<u8\>
```rb
  let s = "hello world";
  let copy = s.as_bytes().to_vec();
```
Vec\<u8\> to String
```rb
  let v = "hello world".as_bytes().to_vec();
  let s = String::from_utf8(v).unwrap();
```
Vec\<u8\> to &str
```rb
  use std::str;
  let v = "hello world".as_bytes().to_vec();
  let s = str::from_utf8(&v).unwrap();
```
String to number
```rb
  let s = "-0.123";
  let f = s.parse::<f32>().unwrap();
```
Number to String
```rb
  let f = -0.123;
  let s = f.to_string();
```
### *Comparison*
```rb
 let s1 = "abc";
 let s2 = "acb";
 let ord = s1.cmp(s2);
 println!("{:?}", ord);
```
Output
```
  Less
```
Both String and &str implement partial ordering. So, it is also possible to compare strings with comparison operator.
```rb
  let s1 = String::from("abc");
  let s2 = String::from("acb");
  assert!(s1 < s2); // ok
```
### *Concatenation*
The string on the left side of addition operator must be owned.
```rb
  let msg = "hello".to_owned() + " world";
  assert!(msg == "hello world"); // ok
```
You can also concatanate a string to an exsiting String with add-assign operator.
```rb
  let mut msg = String::from("hello");
  msg += " world";
  assert!(msg == "hello world"); // ok                 
```
### *Replacing*
Replacing all the matched patterns
```rb
  let s = "foo foo 1 foo";
  let ret = s.replace("foo", "hoo");
  assert!(ret == "hoo hoo 1 hoo"); // ok
```
Replacing the first N matched patterns
```rb
  let s = "foo foo 1 foo";
  let ret = s.replacen("foo", "hoo", 2);
  assert!(ret == "hoo hoo 1 foo"); // ok
```
Replacing the matched pattern at Nth occurrences.
```rb
  let mut s = "foo foo 1 foo".to_string();
  // you need to know the precise range of the string you are replacing in advance.
  s.replace_range(4..7, "hoo");
  assert!(s == "foo hoo 1 foo"); // ok
```
### Split by delim
```rb
  let s = "testing,comma separated,strings";
  let res = s.split(',').collect::<Vec<&str>>();
  let mut iter = res.iter();
  assert!(iter.next() == Some(&"testing"));           // ok
  assert!(iter.next() == Some(&"comma separated"));   // ok
  assert!(iter.next() == Some(&"strings"));           // ok
  assert!(iter.next() == None);                       // ok
```
