mod attractor;

use attractor::*;
use nannou::prelude::*;
use std::process::exit;

const RECORDING: bool = false;

const WINDOW_H: u32 = if RECORDING { 2160 } else { 800 };
const WINDOW_W: u32 = WINDOW_H * 2;

const SUB_WINDOW_H: u32 = 500;
const SUB_WINDOW_W: u32 = 500;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

struct Model {
    window_id: WindowId,
    //attractor: Attractor<LorenzAttractor>,
    attractor: Attractor<HalvorsenAttractor>,
    //attractor: Attractor<ThomasAttractor>,
}

fn model(app: &App) -> Model {
    let id = app.new_window()
        .size(WINDOW_W, WINDOW_H)
        .visible(!RECORDING)
        .view(view)
        .build()
        .unwrap();

    app.new_window()
        .size(SUB_WINDOW_W, SUB_WINDOW_H)
        .view(sub_view)
        .build()
        .unwrap();

    Model {
        window_id: id,
        attractor: Attractor::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let minutes = app.elapsed_frames() / 60 / 60;
    if minutes > 10 {
        exit(0);
    }

    model.attractor.update();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.attractor.draw(&draw);

    if app.elapsed_frames() < 60 * 5 && RECORDING {
        draw.text("←←← Drag or Swipe →→→")
            .width(WINDOW_W as f32)
            .center_justify()
            .font_size(50)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();

    if RECORDING {
        save_frame(app, model.window_id);
    }
}

fn sub_view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GRAY);

    let frame_num = app.elapsed_frames();
    let minutes = frame_num / (60 * 60);
    let seconds = frame_num % (60 * 60) / 60;

    draw.text(&format!(
        "FPS: {:.1}\nframe: {}\n{:0} m {:02} s",
        app.duration.updates_per_second(),
        frame_num,
        minutes,
        seconds,
    ))
    .font_size(40)
    .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

#[allow(dead_code)]
fn save_frame(app: &App, id: WindowId) {
    let frame_num = app.elapsed_frames();

    let path = app
        .project_path()
        .expect("could not locate project_path")
        .join("snapshots")
        .join(frame_num.to_string())
        .with_extension("png");

    app.window(id).unwrap().capture_frame(path);
}
