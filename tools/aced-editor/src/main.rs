use macroquad::{
    color::WHITE, prelude::*, ui::{root_ui, Skin},
};

#[macroquad::main("Aced Engine Editor")]
async fn main() {
    let el_skin = build_skin();
    root_ui().push_skin(&el_skin);

    loop {
        clear_background(WHITE);
        root_ui().label(None, "Aced Engine Editor");
        root_ui().button(None, "Click me");
        next_frame().await;
    }
}

fn build_skin() -> Skin {
    let window_style = root_ui()
        .style_builder()
        .margin(RectOffset { left: 12.0, right: 12.0, bottom: 10.0, top: 10.0 })
        .build();

    let label_style = root_ui()
        .style_builder()
        .color(BLACK)
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(Image::gen_image_color(1, 2, LIGHTGRAY))
        .background_hovered(Image::gen_image_color(1, 2, Color { r: GRAY.r + 0.15, g: GRAY.g + 0.15, b: GRAY.b + 0.15, a: 1.0}))
        .background_clicked(Image::gen_image_color(1, 2, GRAY))
        .margin(RectOffset { left: 6.0, right: 6.0, bottom: 4.0, top: 4.0 })
        .build();

    Skin {
        window_style: window_style,
        label_style: label_style,
        button_style: button_style,
        ..root_ui().default_skin()
    }
}