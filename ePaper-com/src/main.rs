/* main.rs - entry for epaper backend
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

#![feature(allocator_api, vec_into_raw_parts)]
use std::process::Command;
mod terminal;
mod IT8951;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use IT8951::epaper;
fn main(){
    //creates fifo. As the programm will not fail if the file exits, it is not checked
    let _result = Command::new("mkfifo")
        .arg("./updatefifo")
        .status()
        .unwrap();
    let x11_epaper_fifo = 
        OpenOptions::new()
            .read(true)
            .open("../x11_epaper_fifo")
            .unwrap()
        ;
    let mut fifo_reader = BufReader::new(x11_epaper_fifo);
    let mut fifo_line = String::new();

    //sets  resolution of HDMI-1 to the of the connected epaper display

    unsafe{

    let epaper: epaper = epaper::init(1810);
    println!("a:{}",epaper.info.Memory_Addr_L);
    println!("b:{}",epaper.info.Memory_Addr_H);
    println!("target:{}",epaper.gettargetaddr());
    epaper.clear();
    //let _ = Command::new("sleep").arg("5").spawn().unwrap().wait(); 

    //xauth to let root allow to use the display
    //let _ = Command::new("bash").args(["-c","xauth add $(xauth -f ~pk/.Xauthority list | tail -1)"]).spawn().unwrap().wait();
    //let _ = Command::new("./smartclock/build/linux/arm64/release/bundle/smartclock").spawn().expect("smartclock binary not found.");
    ////let _ = Command::new("sleep").arg("10").spawn().unwrap().wait();
    while fifo_reader.read_line(&mut fifo_line).unwrap() > 0 {
        #[cfg(debug_assertions)]
        println!("updating file");
        let buffer= std::fs::read(std::path::Path::new("../image.bin")).unwrap();
        //let buffer= vec![100_u8;1200*825];
        epaper.writeimage(buffer);
        //println!("{}", line);
        fifo_line.clear();
    }

    //let _ = Command::new("wmctrl").args(["-r","smartclock","-b","add,fullscreen"]).spawn().unwrap().wait();
        /*loop {
            //wmctrl sometimes doesn't work. It is also called in run.sh
            //let _ = Command::new("wmctrl").args(["-r","smartclock","-b","add,fullscreen"]).spawn().unwrap().wait();
            //let _rm = Command::new("rm").args(["./screen.png","./output.raw"]).spawn().unwrap().wait();
            //let _scrot = Command::new("scrot").args(["-D",":0","./screen.png"]).spawn().unwrap().wait();

            //let _ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","gray","output.raw"]).spawn().unwrap().wait();
            let buffer= std::fs::read(std::path::Path::new("../image.bin")).unwrap();
            epaper.writeimage(buffer);
            let _ = Command::new("sleep").arg("1").spawn().unwrap().wait();
            //readasynync().await;
    
        //if the programm would not run indefinitly, drop would need to be called seperatly as it also shutdowns the epaper
        }*/
        drop(epaper);
    }  
}
