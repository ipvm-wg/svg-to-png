use resvg::{self, usvg::fontdb::Database};

pub fn rasterize(input: String) -> Vec<u8> {
    // Parse SVG data to usvg tree
    let tree =
        resvg::usvg::Tree::from_str(&input, &resvg::usvg::Options::default(), &Database::new())
            .unwrap();

    // Convert the size of the tree from floats to ints
    let tree_size = tree.size().to_int_size();
    let (width, height) = (tree_size.width(), tree_size.height());

    // Create a pixmap with the tree's dimensions to store the rendered image
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).unwrap();

    // Render the tree to a pixmap
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::default(),
        &mut pixmap.as_mut(),
    );

    // Encode as PNG
    pixmap.encode_png().unwrap()
}
