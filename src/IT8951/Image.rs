/* IT8951/Image.rs - unfinished type system for IT8951 
 *
 * Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
 */


#[derive(Clone, Copy)]
pub enum ImageDepth{
    Full,
    Half,
    Quart,
    Eight,
}
impl std::convert::Into<i8> for ImageDepth{
    fn into(self) -> i8{
        match self{
            ImageDepth::Full => 8,
            ImageDepth::Half => 4,
            ImageDepth::Quart=> 2,
            ImageDepth::Eight=> 1,
        }
    }
}
//todo: create macro for conversion 
//auto wrappers
impl std::convert::Into<u8>    for ImageDepth{fn into(self)->u8{(self as i8)as u8}}
impl std::convert::Into<u16>   for ImageDepth{fn into(self)->u16{(self as i8)as u16}}
impl std::convert::Into<u32>   for ImageDepth{fn into(self)->u32{(self as i8)as u32}}
impl std::convert::Into<i16>   for ImageDepth{fn into(self)->i16{(self as i8)as i16}}
impl std::convert::Into<i32>   for ImageDepth{fn into(self)->i32{(self as i8)as i32}}
impl std::convert::Into<isize> for ImageDepth{fn into(self)->isize{(self as i8)as isize}}


pub struct Image{
    //info:IT8951_Dev_Info,
    pub depth: ImageDepth,
    image: Vec<u8>,
    pub startx: u16,
    pub starty: u16,
    pub width: u16,
    pub height: u16
    
}
impl Image{
    //todo: impl color not as packed but as unpacked variant
    pub fn newImage(depth: ImageDepth,width:u16,height:u16,color: u8) -> Image{
        let image: Vec<u8> = vec![color;(Self::calcimagesize(depth,width,height) as usize).try_into().unwrap()];
        Image { depth, image ,startx: 0, starty: 0, width, height}
    }

    pub fn calcallgignbits(depth: ImageDepth) -> u32{
        (depth as u32)%(8 as u32)
    }
    pub fn allignbits(&self) -> u32{Self::calcallgignbits(self.depth)}

    pub fn calcimagesize(depth: ImageDepth,width: u16,height: u16) -> i32{
        (((width as i32)*(depth as i32)/8)+
            // bit aligning, intern the epaper uses lines and not a Image Vector so padding at the end is required
            match (depth as i32)%8{0=>0,_=>1}
        )*(height as i32) 
    }
    pub fn imagesize(&self) -> i32{Self::calcimagesize(self.depth, self.width, self.height)}

    pub fn pixelpos(&self,posx: u16, posy: u16) -> (u16,u16){ //index, start bit in byte
        //todo impl depth as u16
        (posy*self.width+posx/((self.depth as u8) as u16),posx%(self.depth as u16)*(self.depth as u16))

    }
}

impl std::ops::Index<usize> for Image{
    type Output = u8;
    fn index(&self, index: usize) -> &u8{
         &self.image[index]
    }
}
impl std::ops::IndexMut<usize> for Image{
    fn index_mut(&mut self, index: usize) -> &mut u8{
         &mut self.image[index]
    }
}