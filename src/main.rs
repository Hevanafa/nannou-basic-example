// basic nannou example with a blue frame

use nannou::prelude::*;

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);
    // draw.ellipse().color(WHITE);

    draw.rect()
        .x_y(0.0, 10.0)
        .w_h(100.0, 50.0)
        .color(PLUM);

    draw.rect()
        .x_y(0.0, 10.0)
        .w_h(100.0, 50.0)
        .no_fill()
        .stroke_weight(4.0)
        .stroke(WHITE);

    draw.text("Hello world!")
        .x_y(0.0, 20.0)
        .font_size(14)
        .color(WHITE);

    // flush
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    println!("init nannou");

    nannou::sketch(view).size(320, 240).run();
}
