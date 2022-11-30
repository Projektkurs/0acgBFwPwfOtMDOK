/* xorg.rs - mounts to display with given mode
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

use x11::{xlib,xrandr};
use std::process::Command;

/// searches for a free X11 output, adds the given mode and outputs with that mode
pub unsafe fn mountmodetofree(display: *mut xlib::_XDisplay,resources: *mut xrandr::XRRScreenResources,mode: &str){
    for i in 0..(*resources).noutput{
        let outputs = *((*resources).outputs.offset((i*1).try_into().unwrap()));
        let monitorinfo = xrandr::XRRGetOutputInfo(display,resources,outputs);
        let c_str = std::ffi::CStr::from_ptr((*monitorinfo).name);
        println!("name:{}, connection status:{}",c_str.to_str().unwrap(),(*monitorinfo).connection);
        if (*monitorinfo).connection==1{
            //todo: use X api 
            let _ = Command::new("xrandr").args([
                "-d",
                ":0",
                "--addmode",
                c_str.to_str().unwrap(),
                mode
            ]).spawn().unwrap().wait();
            let _ = Command::new("xrandr").args([
                "-d",
                ":0",
                "--output",
                c_str.to_str().unwrap(),
                "--mode",
                mode
            ]).spawn().unwrap().wait();

            break;
        }
        xrandr::XRRFreeOutputInfo(monitorinfo);
    }
}