use std::path::Path;

use svg_to_png::{
    render::{make_surface_into_dynamic_image, render_image},
    svgs::get_svg_handler,
};

fn main() {
    let handler = get_svg_handler(Path::new("input.svg"));
    let handler = handler.unwrap();
    let surface = render_image(handler.width, handler.height, handler.handle, None);
    assert_eq!(surface.is_ok(), true);
    let image = make_surface_into_dynamic_image(&mut surface.unwrap());
    assert_eq!(image.is_ok(), true);

    let image = image.unwrap();
    let _ = image.save("output.png");
}
