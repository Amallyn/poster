
# Image poster generator

- Generates an image from a folder of images
- Images are aligned in rows.
- The poster will be generated with the width, offset between images and the number of images per row provided.
- Images are placed in alphabetical order.
- Sub images are resized using the same width/height ratio as the first image.
- The image folder should only contain images.

# Usage
```
poster -w poster_width -o offset_between_images -n number of images per row image_folder poster_image_name.jpg
```
Example for a 1022 width poster, 8 pixels offset between images, 3 images per row.
Linux
```
poster -w 1022 -o 8 -n 3 images poster.jpg
```
Windows
```
poster.exe -w 1022 -o 8 -n 3 images poster.jpg
```

# Notes
- A colored background is generated
- Image file format supported are the ones supported by the Rust Crate Image (https://docs.rs/image/latest/image/).
- Quick rewrite in Rust from Python

# To do
- [ ] handle more errors in place of unwrap
- [ ] data types
- [ ] use structs
- [ ] variable scope warnings