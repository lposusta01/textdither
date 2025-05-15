use image::*;
use imageproc::*;

fn downscaled_bit_matrix (img: ImageBuffer/*no clue what this type should actually be*/) -> Vec<Vec<bool>> {
    //theres probably a much simpler way of doing this but idc <3
    let threshold = Luma::from(RgbImage::from([100, 100, 100])); //white threshold
    let mut width = 0;
    let mut height = 0;
    for p in GrayImage::from(img).pixels() {
        if Luma::from(p).0 >= threshold {
            
        }
        if width == img.get
    }
}

fn main() {
    
}