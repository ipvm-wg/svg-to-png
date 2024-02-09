#[test]
fn it_works() {
    let expected = include_bytes!("fixtures/expected.png");
    let svg = r#"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
<circle cx="50" cy="50" r="50" />
</svg>"#;

    let bytes = svg_to_png::rasterize(svg);
    assert_eq!(bytes, expected);
}
