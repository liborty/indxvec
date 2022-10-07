use crate::Printing;
use std::fmt::Write;

/// When interpolated, makes following foreground rendering bold red
pub const RD: &str = "\x1B[1;31m";
/// When interpolated, makes following foreground rendering bold green    
pub const GR: &str = "\x1B[1;32m";
/// When interpolated, makes following foreground rendering bold yellow    
pub const YL: &str = "\x1B[1;33m";
/// When interpolated, makes following foreground rendering bold blue    
pub const BL: &str = "\x1B[1;34m";
/// When interpolated, makes following foreground rendering bold magenta    
pub const MG: &str = "\x1B[1;35m";
/// When interpolated, makes following foreground rendering bold cyan    
pub const CY: &str = "\x1B[1;36m";
/// Returns the terminal rendering to default
pub const UN: &str = "\x1B[0m";

impl<T> Printing<T> for T
where T: std::fmt::Display,
{
    fn to_str(self) -> String { self.to_string() }
    fn to_plainstr(self) -> String { self.to_string() }
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

    fn to_plainstr(self) -> String { 
        match self.len() {
            0 => "".to_string(),
            1 =>  format!("{}",self[0]),   
            _ => self.iter().skip(1).fold(format!("{}",self[0]), |mut s, item| {
            write!(s, " {}", item).ok();  s  })
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

    fn to_plainstr(self) -> String {
        if self.is_empty() { return "".to_string() };
        self.iter().fold("\n".to_string(),
        |mut s, item| { writeln!(s," {}",item.to_str()).ok();  s  })
    } 
}

impl<T> Printing<T> for &[Vec<T>]
where T: std::fmt::Display,
{
    fn to_str(self) -> String {
        if self.is_empty() { return "[]".to_string() };
        self.iter().fold("[\n".to_string(),
        |mut s, item| { 
            writeln!(s," {}",item.to_str()).ok();  s  }) + "]"
    } 

    fn to_plainstr(self) -> String {
        if self.is_empty() { return "".to_string() };
        self.iter().fold("\n".to_string(),
        |mut s, item| { writeln!(s," {}",item.to_str()).ok();  s  })
    } 
}
