/* xorg.rs - mounts to display with given mode
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */
extern crate x11;
use crate::terminal;
use std::ffi::CString;
use std::str::FromStr;
use x11::xrandr::XRRAddOutputMode;
use x11::{xlib, xrandr::*};
use xlib::*;

/// creates a cvt reduced modeinfo
/// note that if the modeinfo is used to create a mode, the id needs to be set afterwards
pub fn createmodeinfo<'a>(width: u32, height: u32, mode_name: &'a CString) -> XRRModeInfo {
    //using command cvt to generate default information for a xrandr mode
    let cvt_output = std::process::Command::new("cvt")
        .args([width.to_string(),height.to_string(),String::from("-r")])
        .output()
        .expect("creating meta information for cvt mode failed, check you have the \"cvt\" command installed.");

    let cvt_str = String::from_utf8_lossy(&cvt_output.stdout);
    let cvt_str_secondline: Vec<&str> = cvt_str.split("\n").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect();
    // contains the values from cvt command as u32
    let mut raw_mode_info: Vec<u32> = Vec::with_capacity(7);
    // as the clock frequency is u64, it is not found in raw_mode_info
    let clock_freq = (f32::from_str(cvt_str_secondline[2]).unwrap() * 1e6) as u64;
    for i in cvt_str_secondline[3..11].iter() {
        raw_mode_info.push(std::str::FromStr::from_str(i).unwrap());
    }
    let name_ptr = mode_name.as_ptr() as *mut std::os::raw::c_char;
    XRRModeInfo {
        name: name_ptr,
        nameLength: (mode_name.to_bytes().len() as u32),
        dotClock: clock_freq,
        width,
        hSyncStart: raw_mode_info[1],
        hSyncEnd: raw_mode_info[2],
        hTotal: raw_mode_info[3],
        height,
        vSyncStart: raw_mode_info[5],
        vSyncEnd: raw_mode_info[6],
        vTotal: raw_mode_info[7],
        modeFlags: 0, // pretty sure it is 0
        id: 0,        // overriden later
        hSkew: 0,     // just needed for crt monitors -> irrelevant
    }
}

/// takes a mode_info and mounts this to a free output using always a free crtc
pub unsafe fn mounttofreeoutput(mut mode_info: XRRModeInfo) -> (RROutput, RRCrtc) {
    // Open display connection.
    let display_name = CString::new(":0").unwrap();
    let display = xlib::XOpenDisplay(display_name.as_ptr());
    if display.is_null() {
        panic!("XOpenDisplay failed. This is propably due to ':0' not being the XID.");
    }

    let window = xlib::XDefaultRootWindow(display);
    let resources = XRRGetScreenResources(display, window);
    if resources.is_null() {
        panic!("failed to obtain resources from XRR");
    }
    terminal::success("Getting XRR Resources");
    println!(
        "    found {} crtc's
             \r    found {} mode's
             \r    found {} output's",
        (*resources).ncrtc,
        (*resources).nmode,
        (*resources).noutput
    );

    let new_mode = XRRCreateMode(display, window, &mut mode_info);
    mode_info.id = new_mode;
    println!("mode XID: {}", mode_info.id);
    terminal::success("Creating new mode");

    // find free output
    let (mut output, freecrtc): (RROutput, Option<RRCrtc>) =
        findfreeoutput(resources, display).unwrap();
    println!("found unused output with XID {}", output);
    // find free crtc
    let crtc: RRCrtc = if let Some(value) = freecrtc {
        println!(
            "unused output has already crtc (XID={}) connected, will use this crtc instead",
            value
        );
        value
    } else {
        // note that it just panics if no crtc is found. It does not retry to find a different output.
        findfreecrtctooutput(resources, display, output)
            .expect("found no crtc which can be connected to the free output")
    };
    XRRAddOutputMode(display, output, mode_info.id);
    terminal::success("Adding mode to output");
    XRRSetCrtcConfig(
        display,
        resources,
        crtc,
        x11::xlib::CurrentTime,
        0,
        0,
        mode_info.id,
        RR_Rotate_0 as u16,
        &mut output,
        1,
    );
    terminal::success("setting crtc config");
    set_output_connected(display, output);
    terminal::success("changing output to active");

    XRRDestroyMode(display, new_mode);
    XRRFreeScreenResources(resources);
    (output, crtc)
    // find free crtc,
    /*let mut freecrtc = 0_u64;
    for i in 0..(*resources).ncrtc {
        let crtc: RRCrtc = *((*resources).crtcs.offset((i).try_into().unwrap()));

        let crtc_info = XRRGetCrtcInfo(display, resources, crtc);

        println!("found crtc: {} with XID {}", i, crtc);
        for u in 0..(*crtc_info).npossible{
        println!("possible: {}",*(*crtc_info).possible.offset(u as isize));
        }
        XRRFreeCrtcInfo(crtc_info);
    }
    for i in 0..(*resources).ncrtc {
        let crtc: RRCrtc = *((*resources).crtcs.offset((i * 1).try_into().unwrap()));

        let crtc_info = XRRGetCrtcInfo(display, resources, crtc);
        if (*crtc_info).noutput == 0 {
            freecrtc = crtc;
            println!("found unused crtc: {} with XID {}", i, crtc);
            break;
        }
        XRRFreeCrtcInfo(crtc_info);
    }*/
    /*
    assert_ne!(freecrtc, 0);
    let mut crtc_info = XRRGetCrtcInfo(display, resources, freecrtc);
    let mut output: RROutput = 0;
    for i in 0..(*resources).noutput {
        output = *((*resources).outputs.offset((i * 1).try_into().unwrap()));
        let output_info = XRRGetOutputInfo(display, resources, output);
        if (*output_info).connection == x11::xrandr::RR_Disconnected as u16 {
            println!("found unused output: {} with XID {}", i, output);
            if (*output_info).crtc!=0{
                println!("unused output has already crtc (XID={}) connected, will use this crtc instead",(*output_info).crtc);
                freecrtc=(*output_info).crtc;
                XRRFreeCrtcInfo(crtc_info);
                crtc_info = XRRGetCrtcInfo(display, resources, freecrtc);
            }
            XRRAddOutputMode(display, output, mode_info.id);
            terminal::success("Adding mode to output");

            //todo: set x and y to a position that is not used by other display
            // If you ask why the first only works on x86(_64)? I honestly do not know, and I got tired of searching for a reason.
            // If you know why, feel free to message me with the Email given in the copyright notice.
            //#[cfg(any(target_arch = "x86",target_arch = "x86_64"))]
            //{
            //set mode to output
            //todo: set x and y to a position that is not used by other display
            XRRSetCrtcConfig(
                display,
                resources,
                freecrtc,
                x11::xlib::CurrentTime,
                0,
                0,
                mode_info.id,
                RR_Rotate_0 as u16,
                &mut output,
                1,
            );
            terminal::success("setting crtc config");
            //}
            //enable mode
            set_output_connected(display, output);
            terminal::success("changing output to active");

            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {}
            XRRFreeOutputInfo(output_info);

            break;
        }
        XRRFreeOutputInfo(output_info);
    }
    assert_ne!(output,0);
    //free up resources
    XRRFreeCrtcInfo(crtc_info);

    XRRDestroyMode(display, new_mode);
    XRRFreeScreenResources(resources);
    (output,crtc)
    */
}
// iterates over the crtcs and returns the first which is able to output to the given output
fn findfreecrtctooutput(
    resources: *mut XRRScreenResources,
    display: *mut _XDisplay,
    output: RROutput,
) -> Option<RRCrtc> {
    unsafe {
        for i in 0..(*resources).ncrtc {
            let crtc: RRCrtc = *((*resources).crtcs.offset((i).try_into().unwrap()));

            let crtc_info = XRRGetCrtcInfo(display, resources, crtc);

            println!("found crtc: {} with XID {}", i, crtc);
            for u in 0..(*crtc_info).npossible {
                if *(*crtc_info).possible.offset(u as isize) == output {
                    return Some(crtc);
                }
            }
            XRRFreeCrtcInfo(crtc_info);
        }
        None
    }
}
/// iterates over the available outputs and returns the first it finds.
/// If this has already a crtc connected to it, it will return also XID of the crtc.
fn findfreeoutput(
    resources: *mut XRRScreenResources,
    display: *mut _XDisplay,
) -> Option<(RROutput, Option<RRCrtc>)> {
    unsafe {
        let mut freecrtc: Option<RRCrtc> = None;
        for i in 0..(*resources).noutput {
            let output: RROutput = *((*resources).outputs.offset((i * 1).try_into().unwrap()));
            let output_info = XRRGetOutputInfo(display, resources, output);
            if (*output_info).connection == x11::xrandr::RR_Disconnected as u16 {
                if (*output_info).crtc != 0 {
                    println!("unused output has already crtc (XID={}) connected, will use this crtc instead",(*output_info).crtc);
                    freecrtc = Some((*output_info).crtc);
                }
                XRRFreeOutputInfo(output_info);

                return Some((output, freecrtc));
            }
            XRRFreeOutputInfo(output_info);
        }
        None
    }
}
/// sets the connection flag of a given output to RR_Connected
unsafe fn set_output_connected(dpy: *mut x11::xlib::Display, output: RROutput) {
    let status_name = std::ffi::CString::new("RR_Connected").unwrap();
    let connected_atom = XInternAtom(dpy, status_name.as_ptr(), False);
    let connected_value = 1;
    let prop_mode = PropModeReplace;

    XRRChangeOutputProperty(
        dpy,
        output,
        connected_atom,
        XA_CARDINAL,
        32,
        prop_mode,
        &connected_value,
        1,
    );
}
