# Rust-String-Complete-Guide

## Preface

### *Motivation*
Coming from the background of C/C++ and a bit of Python and Javascript, I found Rust's string type system a little confusing. So I wrote this down to help understand the Rust's string data type.
### *Expected Level*
Beginner - Intermediate
### *References*
1. [Rust By Example](https://doc.rust-lang.org/rust-by-example/std/str.html)
2. [Rust-doc String](https://doc.rust-lang.org/std/string/struct.String.html#method.split_off)
3. [Rust-doc Str](https://doc.rust-lang.org/std/primitive.str.html#impl-Add%3C%26%27_%20str%3E)


## Introduction

### *Rust's char type*
This is the first caveat when learning Rust's string. Rust's string is UTF-8 by default. A charater in UTF-8 can be either 1, 2, 3 or 4 bytes. Thus, Rust's char is 4 bytes by default. This is different from C/C++'s char data type, which is 1 byte by default.

Example
```rb
println!("size of char = {} bytes", size_of::<char>());
```
Possible output
```
size of char = 4 bytes
```

### String and &str
In Rust, you often see two types of strings, String and &str. According to the [Rust by Example], the difference is 
> A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, >  growable and not null terminated.
&str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a > view into Vec<T>.
