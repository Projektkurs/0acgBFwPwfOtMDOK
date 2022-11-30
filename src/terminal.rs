/* terminal.rs - format for output 
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

use termion::{style, color};

pub fn success(message: &str){
    println!("{}: {}{}Success{}{}",message,color::Fg(color::LightGreen),style::Bold,color::Fg(color::Reset),style::Reset);
}
#[allow(dead_code)]
pub fn failed(message: &str){
    println!("{}: {}{}Failed{}{}",message,color::Fg(color::LightRed),style::Bold,color::Fg(color::Reset),style::Reset);
}