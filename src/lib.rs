use image::EncodableLayout;
use nsvg;

pub fn rasterize(input: &str) -> Vec<u8> {
    // Load the SVG data
    let svg = nsvg::parse_str(input, nsvg::Units::Pixel, 96.0).unwrap();

    // Rasterize the loaded SVG and return an RgbaImage
    let image = svg.rasterize(1.0).unwrap();
    let (width, height) = image.dimensions();

    // Convert image to bytes and encode it as a PNG
    let mut buf: Vec<u8> = vec![];
    nsvg::image::png::PNGEncoder::new(&mut buf)
        .encode(
            image.to_vec().as_bytes(),
            width,
            height,
            nsvg::image::ColorType::RGBA(8),
        )
        .unwrap();

    buf
}
