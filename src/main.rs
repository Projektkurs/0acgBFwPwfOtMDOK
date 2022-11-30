/* main.rs - entry for epaper backend
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

#![feature(allocator_api, vec_into_raw_parts)]
extern crate x11;
use std::ffi::CString;
use x11::xlib;
use std::process::Command;
mod terminal;
mod xorg;
mod IT8951;

use IT8951::epaper;
fn main(){
    //creates fifo. As the programm will not fail if the file exits, it is not checked
    let _result = Command::new("mkfifo")
        .arg("./updatefifo")
        .status()
        .unwrap();
    

    //sets  resolution of HDMI-1 to the of the connected epaper display
    mountdisplay();

    unsafe{

    let epaper: epaper = epaper::init(1810);
    println!("a:{}",epaper.info.Memory_Addr_L);
    println!("b:{}",epaper.info.Memory_Addr_H);
    println!("target:{}",epaper.gettargetaddr());
    epaper.clear();
    let _ = Command::new("sleep").arg("5").spawn().unwrap().wait(); 

    //xauth to let root allow to use the display
    //let _ = Command::new("bash").args(["-c","xauth add $(xauth -f ~pk/.Xauthority list | tail -1)"]).spawn().unwrap().wait();
    let _ = Command::new("./smartclock/build/linux/arm64/release/bundle/smartclock").spawn().expect("smartclock binary not found.");
    let _ = Command::new("sleep").arg("10").spawn().unwrap().wait();
    let _ = Command::new("wmctrl").args(["-r","smartclock","-b","add,fullscreen"]).spawn().unwrap().wait();
        loop {
            //wmctrl sometimes doesn't work. It is also called in run.sh
            //let _ = Command::new("wmctrl").args(["-r","smartclock","-b","add,fullscreen"]).spawn().unwrap().wait();
            let _rm = Command::new("rm").args(["./screen.png","./output.raw"]).spawn().unwrap().wait();
            let _scrot = Command::new("scrot").args(["-D",":0","./screen.png"]).spawn().unwrap().wait();

            let _ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","gray","output.raw"]).spawn().unwrap().wait();
            let buffer= std::fs::read(std::path::Path::new("./output.raw")).unwrap();
            epaper.writeimage(buffer);
            let _ = Command::new("sleep").arg("50").spawn().unwrap().wait();
            //readasynync().await;
    
        //if the programm would not run indefinitly, drop would need to be called seperatly as it also shutdowns the epaper
        }
        drop(epaper);
    }  
}

fn mountdisplay() {
    unsafe {
        let _handle = Command::new("bash").args(["-c","xrandr -d :0 --newmode \"1200x825R\"   69.00  1200 1248 1280 1360  825 828 838 849 +hsync -vsync"]).spawn().unwrap().wait();
        // Open display connection.
        #[allow(temporary_cstring_as_ptr)] //the deallocation at the end is fine here as XOpenDisplay doesn't use the pointer after the call,
        let display = xlib::XOpenDisplay(CString::new(":0").unwrap().as_ptr());
        //let display = xlib::XOpenDisplay(std::ptr::null());
        
        if display.is_null() {
            panic!("XOpenDisplay failed. This is propably due to ':0' not being the XID.");
        }

        let window= xlib::XDefaultRootWindow(display);
        let resources= x11::xrandr::XRRGetScreenResources(display,window);
        if resources.is_null(){
            panic!("failed to obtain resources from XRR");
        }
        terminal::success("Getting XRR Resources");
        println!(
             "    found {} crtc's
            \r    found {} mode's
            \r    found {} output's",
            (*resources).ncrtc,(*resources).nmode,(*resources).noutput
        );

        //debug information
        /*for i in 0..(*resources).ncrtc{
            let crtc = *((*resources).crtcs.offset((i*1).try_into().unwrap()));

            let crtcinfo = xrandr::XRRGetCrtcInfo(display,resources,crtc);

            println!("width:{},height:{}",(*crtcinfo).width,(*crtcinfo).height);

            xrandr::XRRFreeCrtcInfo(crtcinfo);
        }*/
        xorg::mountmodetofree(display,resources,"1200x825R");
    }
}
