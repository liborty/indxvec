use crate::{GR,UN,Printing};

use std::fmt::Write;

impl<T> Printing<T> for T
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.to_string()
    }
}

impl<T> Printing<T> for &[T]
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.iter().fold(String::from("["), |mut s, item| {
            write!(s, " {}", item).ok();
            s
        }) + " ]"
    }
}

impl<T> Printing<T> for &[&[T]]
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.iter().fold(String::from("["), |mut s, &item| {
            writeln!(s, " {}", item.to_str()).ok();
            s
        }) + "]"
    }
}

impl<T> Printing<T> for &[Vec<T>]
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.iter().fold(String::from("[\n"), |mut s, item| {
            writeln!(s, " {}", item.to_str()).ok();
            s
        }) + "]"
    }
}

/// This just prints the items one by one instead of serializing
pub fn printvv<T>(s: &[Vec<T>])
where
    T: Copy + std::fmt::Display,
{
    println!("{GR}[");
    for v in s {
        println!(" {}", v.to_str())
    }
    println!("]{UN}");
}
