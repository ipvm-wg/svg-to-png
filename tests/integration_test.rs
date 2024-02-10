#[test]
fn it_works() {
    let input = include_str!("fixtures/example.svg");
    let expected = include_bytes!("fixtures/expected.png");
    let actual = svg_to_png::rasterize(input.to_string());

    assert_eq!(actual, expected);
}
