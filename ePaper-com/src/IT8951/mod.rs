/* IT8951/mod.rs - raw function for IT8951
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//might want to be deactivated
#![allow(improper_ctypes)]
#![allow(dead_code)]

mod Image;
//use std::convert::FloatToInt;
//pub type epaper = IT8951_Dev_Info;
use internal::*;
pub mod internal{
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub struct epaper{
    pub info: IT8951_Dev_Info,
    pub initmode: u16,
    pub epdmode: u16,
    pub amode: u8
}

impl epaper{

    pub unsafe fn init(vcom: u16) -> epaper{
        DEV_Module_Init();
        epaper{
            info: EPD_IT8951_Init(vcom),
            initmode: 0,
            epdmode: 0,
            amode:6
        }
    }
    pub unsafe fn clear(&self){
            EPD_IT8951_Clear_Refresh(self.info, self.gettargetaddr(), 0);
    }
    
    pub unsafe fn sleep(){
        EPD_IT8951_Sleep();
    }

    pub unsafe fn standby(){
        EPD_IT8951_Standby();
    }
    pub unsafe fn wakeup(){
        EPD_IT8951_SystemRun();
    }
    pub unsafe fn writeimage(&self, image: Vec<u8>){
        let (ptr,_,_)= image.into_raw_parts();
        //EPD_IT8951_1bp_Refresh(ptr,0,0,self.info.Panel_W,self.info.Panel_H,self.amode,self.gettargetaddr(),true);
        EPD_IT8951_8bp_Refresh(ptr,0,0,self.info.Panel_W,self.info.Panel_H,false,self.gettargetaddr());

    }
    pub fn gettargetaddr(&self) -> u32{
        <u16 as Into<u32>>::into(self.info.Memory_Addr_L) | <u16 as Into<u32>>::into(self.info.Memory_Addr_H) << 16
    }
    
    pub fn getimagesize(&self) -> u32{
        match self.info.Panel_W * 4 % 8{
            0 => <u16 as Into<u32>>::into(self.info.Memory_Addr_L)*4/8 *<u16 as Into<u32>>::into(self.info.Memory_Addr_H),
            _ => (<u16 as Into<u32>>::into(self.info.Memory_Addr_L)*4/8+1)*<u16 as Into<u32>>::into(self.info.Memory_Addr_H),
        }
    }
    

}
impl Drop for epaper{
    fn drop(&mut self){
        unsafe{
            self.clear();
            EPD_IT8951_Sleep();
            DEV_Module_Exit();
        }
    }
}

pub unsafe fn InitEpaper(){
    
    internal::DEV_Module_Init();
    
    //let definfo = internal::EPD_IT8951_Init();
}