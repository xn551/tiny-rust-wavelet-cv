use image::{ImageBuffer, GrayImage};

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let (a, b) = (2, 3);
    println!( "sum {}", sum(a, b) );

    let dynamic_image = image::open("1a.jpg").unwrap();

// Create a new ImgBuf with width: imgx and height: imgy
    let mut buf_u8: [u8;480000] = [0;480000];

    let gba_image = dynamic_image.into_rgb8();
    let (width, height) = gba_image.dimensions();
    let mut img: GrayImage = ImageBuffer::new(width/2, height/2);



    let mut hh:u16;
    for x in 0..width/2 {
        for y in 0..height/2 {
            //let pixel = gba_image.get_pixel_mut(x, y);
            let pixel_11 = gba_image[(2*x,2*y)];
            let pixel_11g = pixel_11[0] as u16;

            let pixel_12 = gba_image[(2*x,2*y + 1)];
            let pixel_12g = pixel_12[0] as u16;

            let pixel_21 = gba_image[(2*x + 1,2*y)];
            let pixel_21g = pixel_21[0] as u16;

            let pixel_22 = gba_image[(2*x + 1,2*y + 1)];
            let pixel_22g = pixel_22[0] as u16;

            if (pixel_11g + pixel_22g) > (pixel_12g + pixel_21g) {
                hh = (pixel_11g + pixel_22g) - (pixel_12g + pixel_21g);
            }
            else {
                hh = (pixel_12g + pixel_21g) - (pixel_12g + pixel_21g);
            }

            buf_u8[(y*800 + x) as usize ] = (hh*8) as u8 ;
            img.put_pixel(x,y,image::Luma([(hh*8) as u8]));
        }
    }

    img.save("wtl.jpg").unwrap();
    image::save_buffer("image.jpg", &buf_u8, 800, 600, image::ColorType::L8).unwrap()
}
