wit_bindgen::generate!({
    world: "svg-to-png",
    exports: {
        world: Component,
    }
});

use anyhow::Context;
use resvg::{self, usvg::fontdb::Database};
#[cfg(target_arch = "wasm32")]
use wasi::logging::logging::{log, Level};

pub struct Component;

impl Guest for Component {
    fn rasterize(input: String) -> Vec<u8> {
        #[cfg(target_arch = "wasm32")]
        log(Level::Info, "fission:svg-to-png", "rasterizing SVG to PNG");

        match rasterize(input) {
            Ok(png) => {
                #[cfg(target_arch = "wasm32")]
                log(
                    Level::Info,
                    "fission:svg-to-png",
                    "PNG generated successfully!",
                );

                png
            }
            Err(err) => {
                #[cfg(target_arch = "wasm32")]
                log(Level::Error, "fission:svg-to-png", err.to_string().as_str());

                panic!();
            }
        }
    }
}

pub fn rasterize(input: String) -> anyhow::Result<Vec<u8>> {
    // Parse SVG data to usvg tree
    let tree =
        resvg::usvg::Tree::from_str(&input, &resvg::usvg::Options::default(), &Database::new())?;

    // Convert the size of the tree from floats to ints
    let tree_size = tree.size().to_int_size();
    let (width, height) = (tree_size.width(), tree_size.height());

    // Create a pixmap with the tree's dimensions to store the rendered image
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(width, height).context("Failed to create pixmap")?;

    // Render the tree to a pixmap
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::default(),
        &mut pixmap.as_mut(),
    );

    // Encode as PNG
    Ok(pixmap.encode_png()?)
}
