use x11::xlib;

use std::{
    ffi::c_uchar,
    io::{Write,Seek},
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
    vec,
};

/// Takes a screenshot of the root window and converts this image to a raw 8bit grayscale buffer
///
/// This buffer is writen to imagefile and b"rewrite\n" is writen to the given fifo.
/// Both files are just locked for there respected write operation.
/// However, too many screenshots should not be taken at the same time, as this could lead to x11 crashing
pub fn updateepaper(imagefile: &mut std::fs::File, fifo: &mut std::fs::File) {
    let total = Instant::now();
    let mut ms = Instant::now();
    // Open a connection to the X11 server
    let display = unsafe { xlib::XOpenDisplay(std::ptr::null()) };

    // Get the root window
    let root_window = unsafe { xlib::XDefaultRootWindow(display) };

    // Get the dimensions of the root window
    let width = unsafe { xlib::XDisplayWidth(display, 0) as usize };
    let height = unsafe { xlib::XDisplayHeight(display, 0) as usize };

    ms = printtimedif(ms, "Startup X:");

    // Take the screenshot
    let ximage: *mut xlib::XImage = unsafe {
        xlib::XGetImage(
            display,
            root_window,
            0,
            0,
            width as u32,
            height as u32,
            xlib::XAllPlanes(),
            xlib::ZPixmap,
        )
    };

    ms = printtimedif(ms, "XGetImage:");

    // Create an image buffer to store the screenshot
    let mut vec_buffer: Vec<u8> = vec![0 as u8; width * height];

    // Disassemble the buffer
    let (vec_ptr, vec_length, vec_capacity) = vec_buffer.into_raw_parts();

    let ximage_data_ptr = unsafe { ((*ximage).data as *const i32) as usize };
    let vec_buffer_ptr = vec_ptr as usize;
    let threadnum = 4;

    // Convert colored image into a raw array of u8 representing its brightness.
    // As the write buffer uses a single memory allocation, mem::transmute is used instead of
    // a rust feature like slice, to gurantee that the memory is not freed more than once.
    // a mutex would be suboptimal as the threads work on the data at the same time.
    let mut threads = Vec::new();
    for i in 0..threadnum {
        threads.push(thread::spawn(move || {
            for x in (width / threadnum * i)..(width / threadnum * (i + 1)) {
                for y in 0..height {
                    let (r, g, b) = {
                        let data_ptr: *const i32 = unsafe { std::mem::transmute(ximage_data_ptr) };
                        let pixel = unsafe { *data_ptr.offset((y * width + x) as isize) };
                        (
                            (pixel >> 16) as c_uchar,
                            (pixel >> 8) as c_uchar,
                            pixel as c_uchar,
                        )
                    };
                    let brightness = (((r as u16) + (g as u16) + (b as u16)) / 3) as c_uchar;
                    //let brightness = r;
                    let offset = (y * width + x) as usize;
                    unsafe {
                        std::ptr::write(std::mem::transmute(vec_buffer_ptr + offset), brightness);
                    }
                }
            }
        }));
    }

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }
    ms = printtimedif(ms, "thread conversion:");
    // Reassamble the buffer
    vec_buffer = unsafe { Vec::from_raw_parts(vec_ptr, vec_length, vec_capacity) };
    ms = printtimedif(ms, "rassamble buffer:");

    // Write the buffer into an imagebuffer
    //{
        let file = imagefile;//.lock().unwrap();
        file.set_len(0).unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(vec_buffer.as_slice()).unwrap();
        ms = printtimedif(ms, "write image:");
    //}
    //{
        let file = fifo;//.lock().unwrap();
        println!("writing to file");
        file.write(b"rewrite\n").unwrap();
    //}
    // Clean up
    unsafe {
        xlib::XDestroyImage(ximage);
        xlib::XCloseDisplay(display);
    }

    printtimedif(ms, "clean up:");
    printtimedif(total, "total time:");
}

// Print the time each step takes
#[cfg(feature = "printtime")]
fn printtimedif(last_call: Instant, message: &str) -> Instant {
    let now = Instant::now();
    let time_diff = now.duration_since(last_call);
    println!("{}\n\t {} us.", message, time_diff.as_micros());
    now
}

#[cfg(not(feature = "printtime"))]
#[allow(unused_variables)]
fn printtimedif(last_call: Instant, message: &str) -> Instant {
    last_call
}
