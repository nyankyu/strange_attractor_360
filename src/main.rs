mod attractor;

use nannou::prelude::*;
use std::process::exit;
use attractor::*;

const RECORDING: bool = false;

const WINDOW_H: u32 = if RECORDING { 2160 } else { 800 };
const WINDOW_W: u32 = WINDOW_H * 2;

const SUB_WINDOW_H: u32 = 500;
const SUB_WINDOW_W: u32 = 500;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

struct Model {
    attractor: Attractor<LorenzAttractor>,
    //attractor: Attractor<HalvorsenAttractor>,
    //attractor: Attractor<ThomasAttractor>,
    minutes: u64,
}

fn model(app: &App) -> Model {
    app.new_window()
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
        attractor: Attractor::new(),
        minutes: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let minutes = app.elapsed_frames() / 60 / 60;
    if model.minutes < minutes {
        model.minutes = minutes;
        print!("{}, ", minutes);
    }
    if minutes >= 5 {
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
        save_frame(app);
    }
}

fn sub_view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GRAY);

    let fps = app.duration.updates_per_second();
    draw.text(&format!("{:.1}", fps))
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

#[allow(dead_code)]
fn save_frame(app: &App) {
    let frame_num = app.elapsed_frames();

    let path = app
        .project_path()
        .expect("could not locate project_path")
        .join("snapshots")
        .join(frame_num.to_string())
        .with_extension("png");

    app.main_window().capture_frame(path);
}
