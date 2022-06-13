use crate::{GR,UN,Printing};
use std::fmt::Write;

impl<T> Printing<T> for T
where T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.to_string()
    }
}

impl<T> Printing<T> for &[T]
where T: std::fmt::Display, 
{
    fn to_str(self) -> String { 
    match self.len() {
        0 => "[]".to_string(),
        1 =>  format!("[{}]",self[0]),   
        _ => self.iter().skip(1).fold(format!("[{}",self[0]), |mut s, item| {
        write!(s, " {}", item).ok();  s  }) + "]" 
        }
    }
}

impl<T> Printing<T> for &[&[T]]
where T: std::fmt::Display,
{ 
    fn to_str(self) -> String {
        if self.is_empty() { return "[]".to_string() };
        self.iter().fold("[\n".to_string(),
        |mut s, item| { writeln!(s," {}",item.to_str()).ok();  s  }) + "]" 
        } 
}

impl<T> Printing<T> for &[Vec<T>]
where T: std::fmt::Display,
{
    fn to_str(self) -> String {
        if self.is_empty() { return "[]".to_string() };
        self.iter().fold("[\n".to_string(),
        |mut s, item| { writeln!(s," {}",item.to_str()).ok();  s  }) + "]" 
        } 
}

/// This just prints the items one by one instead of serializing
pub fn printvv<T>(s: &[Vec<T>])
where
    T: Copy + std::fmt::Display,
{
    println!("{GR}[");
    for v in s { println!(" {}", v.to_str()) }
    print!("]{UN}");
}
