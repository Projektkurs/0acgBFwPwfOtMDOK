/* main.rs - small webserver to update display
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

 #[macro_use] extern crate rocket;
use std::io::Write;
extern crate libc;

//prints out input into config and writes "generalconfig" into the updatefifo
#[post("/", data = "<data>")]
async fn writegconf(data: String){
    let mut _file = std::fs::File::create("../config").unwrap();
    writeln!(&mut _file, "{}",data).unwrap();

    writeln!(&mut std::fs::File::create("../updatefifo").unwrap(), "{}","generalconfig").unwrap();
}

//prints out input into configs/defaultconfig and writes "config" into the updatefifo
#[post("/", data = "<data>")]
async fn writeconf(data: String){
    let mut _file = std::fs::File::create("../configs/defaultconfig").unwrap();
    writeln!(&mut _file, "{}",data).unwrap();

    writeln!(&mut std::fs::File::create("../updatefifo").unwrap(), "{}","config").unwrap();
}


#[launch]
fn rocket() -> _ {
    //creates fifo. As the programm will not fail if the file exits, it is not checked
    let path = std::ffi::CString::new("myfifo").unwrap();
    unsafe{libc::mkfifo(path.as_ptr(), 0o644)};

    rocket::build()
        .mount("/config", routes![writeconf])
        .mount("/generalconfig", routes![writegconf])
}