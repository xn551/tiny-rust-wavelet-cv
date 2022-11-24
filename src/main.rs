use image::{ImageBuffer, GrayImage};

fn main() {
    let IMG_NAME = "your_figure.jpg";    // support any format png, bmp et al.

    let dynamic_image = image::open(IMG_NAME).unwrap();

// Create a new ImgBuf with width: imgx and height: imgy
    let mut l_buf_u8: [u8;480000] = [0;480000];
    
    let mut hv_buf_u8: [u8;480000] = [0;480000];
    let mut hh_buf_u8: [u8;480000] = [0;480000];
    let mut hd_buf_u8: [u8;480000] = [0;480000];

    let gba_image = dynamic_image.into_rgb8();  // could also use dynamic_image.into_l8()
    let (width, height) = gba_image.dimensions();
    let mut img: GrayImage = ImageBuffer::new(width/2, height/2);

    let mut hh:u16;
    
    for x in 0..width/2 {
        for y in 0..height/2 {
            //let pixel = gba_image.get_pixel_mut(x, y);
            let pixel_11 = gba_image[(2*x,2*y)];
            let pixel_11g = pixel_11[0] as u16;   //  only transfrom the red value, if gba_image is gray image, just delete the [0], [1], [2]

            let pixel_12 = gba_image[(2*x,2*y + 1)];
            let pixel_12g = pixel_12[0] as u16;

            let pixel_21 = gba_image[(2*x + 1,2*y)];
            let pixel_21g = pixel_21[0] as u16;

            let pixel_22 = gba_image[(2*x + 1,2*y + 1)];
            let pixel_22g = pixel_22[0] as u16;

            // not multiply with sqrt(2).
            h_diag = (pixel_11g + pixel_22g) - (pixel_12g + pixel_21g);
            l   = (pixel_11g + pixel_22g) + (pixel_12g + pixel_21g);
            h_vert = (pixel_11g + pixel_12g) - (pixel_21g + pixel_22g);
            h_hori =  (pixel_11g + pixel_21g) - (pixel_12g + pixel_22g);
           
            hd_buf_u8[(y*800 + x) as usize ] = (h_diag) as u8 ;          
            l_buf_u8[(y*800 + x) as usize ] = (l/4) as u8 ;
            hv_buf_u8[(y*800 + x) as usize ] = (h_vert) as u8 ;
            hh_buf_u8[(y*800 + x) as usize ] = (h_hori) as u8 ;
            // to convert to image, shift up the negative value.
            img.put_pixel(x,y,image::Luma([(h_diag + 128) as u8]));
        }
    }

    img.save("wtl.jpg").unwrap();
    // values in l are always positive, so directly save it as image file.
    image::save_buffer("image.jpg", &l_buf_u8, 800, 600, image::ColorType::L8).unwrap()
}
