/* main.rs - entry for epaper backend
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

#![feature(vec_into_raw_parts, slice_pattern)]
extern crate x11;
use rand::distributions::{Alphanumeric, DistString};
mod terminal;
mod ximage;
mod xmount;
use std::fs::{File, OpenOptions};
//use std::io::prelude::*;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ops::Deref;
use std::ops::DerefMut;
fn main() {
    let width = 1200;
    let height = 825;
    // the programm crashes if a mode with the name already exists. This is for example the case if restarting
    // adding a random String at the end is sufficient for now.
    if true {
        let randomstring = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        let mode_name =
            std::ffi::CString::new(format!("{width}x{height}R{randomstring}",)).unwrap();
        let mode_info = xmount::createmodeinfo(width, height, &mode_name);
        let (_output, _) = unsafe { xmount::mounttofreeoutput(mode_info) };
    }
    //todo: updateepaper here
    let lastupdate = Arc::new(Mutex::new(SystemTime::now()));
    let imagefile = Arc::new(Mutex::new(File::create("../image.bin").unwrap()));
    #[cfg(debug_assertions)]
    println!("opening fifo");
    let x11_epaper_fifo = Arc::new(Mutex::new(
        OpenOptions::new()
            .write(true)
            //.custom_flags(libc::O_NONBLOCK)
            .open("../x11_epaper_fifo")
            .unwrap(),
        //OpenOptions::new().read(false).open("x11_epaper_fifo").unwrap(),
    ));
    #[cfg(debug_assertions)]
    println!("opened fifo");
    // let x11_flutter_fifo = Arc::new(Mutex::new(File::create("x11_flutter_fifo").unwrap()));
    // let x11_web_fifo = Arc::new(Mutex::new(File::create("web_x11_fifo").unwrap()));
    //sleeps for one second
    ximage::updateepaper(imagefile.lock().unwrap().deref_mut(), x11_epaper_fifo.lock().unwrap().deref_mut());
    let x11_epaper_fifo_c = Arc::clone(&x11_epaper_fifo);
    let imagefile_c = Arc::clone(&imagefile);
    let time_update: thread::JoinHandle<()> = thread::spawn(move || {
        loop {
            let start_time = Instant::now();
            //holding lastupdate for minimum duration
            {
                let mut lastupdate = lastupdate.lock().unwrap();

                let last_since_unix = lastupdate.duration_since(UNIX_EPOCH).unwrap();
                let now_since_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                //every new minute do a update
                #[cfg(debug_assertions)]
                println!(
                    "now vs last since unix:{}",
                    now_since_unix.as_secs() - last_since_unix.as_secs()
                );
                if now_since_unix.as_secs() / 60 > last_since_unix.as_secs() / 60 {
                    //ximage::updateepaper(imagefile.clone(), x11_epaper_fifo.clone());
                    ximage::updateepaper(imagefile_c .lock().unwrap().deref_mut(), x11_epaper_fifo_c.lock().unwrap().deref_mut());
                    *lastupdate = SystemTime::now();
                }
            }
            let end_time = Instant::now();
            let elapsed = end_time.duration_since(start_time);
            thread::sleep(Duration::from_secs(1) - elapsed);
        }
    });
    let webserver_update: thread::JoinHandle<()> =thread::spawn(move || {
        println!("open webserver_fifo");
        let webserver_fifo = 
        OpenOptions::new()
            .read(true)
            //.custom_flags(libc::O_NONBLOCK)
            .open("../webserver_x11_fifo")
            .unwrap();
        println!("opened webserver_fifo");
        let mut fifo_reader = BufReader::new(webserver_fifo);
        let mut fifo_line = String::new();
        while fifo_reader.read_line(&mut fifo_line).unwrap() > 0 {
            println!("read from webserver_x11_fifo");
            //ximage::updateepaper(imagefile.clone(), x11_epaper_fifo.clone());
            ximage::updateepaper(imagefile.lock().unwrap().deref_mut(), x11_epaper_fifo.lock().unwrap().deref_mut());
            fifo_line.clear();
        }


    }) ;
    //let handles: thread::JoinHandle<()> =thread::spawn(move || {}) ;
    time_update.join().unwrap();
    webserver_update.join().unwrap();
}
