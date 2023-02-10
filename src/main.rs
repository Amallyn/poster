use std::io;
use std::env;
use std::fs;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbaImage, imageops};

// TODO: handle errors in place of unwrap, data types, scopes, use structs

fn main()-> io::Result<()> {
    let usage = String::from("Invalid Arguments.\nUsage: poster -w poster_width -o offset_between_images -n number of images per row image_folder poster_image_name.jpg\nEg. guild-poster -w 1022 -o 8 -n 3 images alliance.jpg");
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len < 3 {
        panic!("{}",usage);
    }

    let img_dir = &args[args_len-2];
    let poster_name = &args[args_len-1];

    // final poster image use std::io;width
    let mut width: u32 = 1022;
    // height calculated later depending on sub image ratio
    let mut height: u32 = width;
    // default, should be newly allocated later
    let mut poster: RgbaImage = ImageBuffer::new(4, 4);
    // sub image rows
    let mut rows: u32 = 1;

    // number of pixels between sub images
    let mut offset: u32 = 8;
    // number of sub images per row
    let mut img_per_row: u32 = 3;

    // sub image width/heigth ratio already set?
    let mut ratio_set = false;
    let mut ratio: f32 = 1.0;

    let mut i =1;

    while i < args_len - 3 {
        match args[i].as_str() {
            "-w" => {
                width = args[i+1].parse().expect("Not a number!");
            }
            "-o" => {
                offset = args[i+1].parse().expect("Not a number!");
            }
            "-n" => {
                img_per_row = args[i+1].parse().expect("Not a number!");
            }
            &_ => println!("{}",usage),
        }
        i+=2;
    }

    dbg!(width, offset, img_per_row, img_dir, poster_name);
    let w_offset: u32 = offset;
    let h_offset: u32 = offset;
    let subimage_width: u32 = (width-((1+img_per_row)*w_offset))/img_per_row;
    // will be calculated from first sub image ratio
    let mut subimage_height: u32 = subimage_width;
    //dbg!(subimage_width);

    //let img_paths = fs::read_dir(img_dir).unwrap();
    let mut img_paths = fs::read_dir(img_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    img_paths.sort();
    dbg!(&img_paths);
    // second read_dir call
    let path_count = fs::read_dir(img_dir).unwrap().count();

    i = 0;
    for img_path in img_paths {
        //println!("Name: {}", img_fname.unwrap().path().display());
        let img_path_name = img_path;
        //let img_path_name = img_path.unwrap().path();
        let img = image::open(&img_path_name).unwrap();
        println!("Name: {}", &img_path_name.display());
        println!("dimensions {:?}", img.dimensions());
        if ! ratio_set {
            // inverted ratio: height/width
            ratio = img.dimensions().1 as f32 / img.dimensions().0 as f32;
            ratio_set = true;
            subimage_height = (subimage_width as f32 * ratio) as u32;
            // only images expected in image_dir
            // calculate rows based on img_per_row and number of images
            rows = (path_count as u32+(img_per_row-1))/ img_per_row;
            // calculate height based on first sub image and offsets between images
            height = (rows+1) * h_offset  + rows * subimage_height;
            // fill with color for now
            poster = ImageBuffer::from_fn(width, height, |_x, _y| {
                image::Rgba([240, 240, 240, 255])
            });
            
            dbg!(ratio, path_count, rows, width, height, subimage_width, subimage_height);
        }
        let resized = image::imageops::resize(&img, subimage_width, subimage_height, image::imageops::FilterType::Lanczos3);
        // image column and row
        let col: u32 = i as u32 %img_per_row;
        let row: u32 = i as u32/ img_per_row;
        let x: i64 = (w_offset*(col+1)+subimage_width*col).into();
        let y: i64 = (h_offset*(row+1)+subimage_height*row).into();
        image::imageops::replace(&mut poster, &resized,
            x, y
        );
        dbg!(col, row, x, y);
        i+=1;
           
    }
    poster.save(poster_name).unwrap();

    Ok(())
}
