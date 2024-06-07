use crate::house::room::House;
use std::path::Path as p;
use svg::node::element::path::Data;
use svg::node::element::{Path, Text};
use svg::Document;

pub fn create_svg(mut house: House) {
    let mut document = Document::new()
        .set("width", 1600)
        .set("height", 900)
        .set("viewBox", (0, 0, 1600, 900));

    house.generate_walls();

    let print_x = 1550.0 / house.width.unwrap();
    let print_y = 850.0 / house.length.unwrap();

    //Add to document the walls and windows of the house
    for room in &house.rooms {
        for wall in room.walls.iter() {
            let start = wall.start.clone();
            let end = wall.end.clone();

            let data = Data::new()
                .move_to((start.x * print_x, start.y * print_y))
                .line_to((end.x * print_x, end.y * print_y));

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 2)
                .set("d", data);

            document = document.add(path);
        }

        for window in room.windows.iter() {
            let start = window.start.clone();
            let end = window.end.clone();

            let data = Data::new()
                .move_to((start.x * print_x, start.y * print_y))
                .line_to((end.x * print_x, end.y * print_y));

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "blue")
                .set("stroke-width", 2)
                .set("d", data);

            document = document.add(path);
        }
    }

    // Add to document the doors of the house
    for door in &house.doors {
        if let (Some(start), Some(end)) = (&door.start, &door.end) {
            let data = Data::new()
                .move_to((start.x * 100.0, start.y * 100.0))
                .line_to((end.x * 100.0, end.y * 100.0));

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "red")
                .set("stroke-width", 2)
                .set("d", data);
            document = document.add(path);
        }
    }

    // Add to document the name of the rooms of the house in the middle of each room
    for room in &house.rooms {
        let x = room.position.as_ref().unwrap().x * print_x + room.width.unwrap() / 2.0 * print_x;
        let y = room.position.as_ref().unwrap().y * print_y + room.length.unwrap() / 2.0 * print_y;
        let text = Text::new(room.name.clone())
            .set("x", x)
            .set("y", y)
            .set("text-anchor", "middle")
            .set("font-size", 20);

        document = document.add(text);
    }

    //Save the document to a file
    svg::save("src/assets/.plan.svg", &document).unwrap();
}

pub fn render_svg_to_png(svg_file: &str, filepath: &str) {
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(svg_file)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    opt.keep_named_groups = true;
    opt.fontdb.load_system_fonts();
    let fit_to = usvg::FitTo::Zoom(1.0);

    let svg_data = std::fs::read(svg_file).unwrap();
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

    let pixmap_size = fit_to
        .fit_to(rtree.svg_node().size.to_screen_size())
        .unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&rtree, fit_to, pixmap.as_mut()).unwrap();
    pixmap.save_png(p::new(filepath)).unwrap();
}
