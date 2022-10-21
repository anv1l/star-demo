mod star;

use nannou::prelude::*;
use rand::prelude::*;
pub struct Model {
    pub stars: Vec<star::Star>,
}
fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let boundary = _app.window_rect();

    let mut stars: Vec<star::Star> = vec![];

    stars.resize_with(10000, || {
        star::Star::new(boundary.w(), boundary.h(), &mut thread_rng())
    });

    Model { stars }
}

fn update_all(app: &App, model: &mut Model) {
    let boundary = app.window_rect();
    for star in model.stars.iter_mut() {
        star.update(boundary.w(), boundary.h(), 0.5, 1.0, &mut thread_rng())
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    update_all(_app, _model);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();
    draw.background().color(BLACK);

    for star in &_model.stars {
        let percent = 1.0 - star.position.z / boundary.w();
        let radius = star.radius * percent;

        if star.position.z > 0.0 {
            draw.rect()
                .color(Rgba::new(207.0, 201.0, 235.0, percent * 255.0))
                .w(radius)
                .h(radius)
                .x_y(star.position.x + app.mouse.x, star.position.y + app.mouse.y);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
