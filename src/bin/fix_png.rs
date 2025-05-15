use glob::glob;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

fn main() {
    for entry in glob("assets/images/*.png").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let img = image::open(path.clone()).unwrap();
                let dimensions = img.dimensions();
                let mut buf = ImageBuffer::<Rgba<u8>, _>::new(dimensions.0, dimensions.1);

                let mut modified = false;
                for (x, y, pixel) in img.pixels() {
                    // Update all background pixels to match nearest non-transparent pixel.
                    if pixel == Rgba([255, 255, 255, 0]) {
                        modified = true;
                        if let Some(nearest_pixel) =
                            find_nearest_non_transparent_pixel(&img, (x, y))
                        {
                            buf.put_pixel(
                                x,
                                y,
                                Rgba([nearest_pixel[0], nearest_pixel[1], nearest_pixel[2], 0]),
                            );
                        } else {
                            // Mark as black to indicate it has been processed so we can skip in the future.
                            buf.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                        }
                    } else {
                        buf.put_pixel(x, y, pixel);
                    }
                }

                if modified {
                    println!("Updated file: {:?}", path);
                    buf.save(path).unwrap();
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn find_nearest_non_transparent_pixel(img: &DynamicImage, point: (u32, u32)) -> Option<Rgba<u8>> {
    for y in -1..=1 {
        for x in -1..=1 {
            let new_x = (point.0 as i32 + x) as u32;
            let new_y = (point.1 as i32 + y) as u32;

            if new_x < img.width() && new_y < img.height() {
                let pixel = img.get_pixel(new_x as u32, new_y as u32);
                if pixel[3] != 0 {
                    return Some(pixel);
                }
            }
        }
    }
    None
}
