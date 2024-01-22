use miniquad::graphics::GraphicsContext;
use miniquad::EventHandler;

struct App;

impl EventHandler for App {
    fn draw(&mut self, gctx: &mut GraphicsContext) {
        gctx.clear(Some((0.2, 0.3, 0.3, 1.0)), None, None);
    }

    fn update(&mut self, _gctx: &mut GraphicsContext) {}
}

fn main() {
    miniquad::start(
        miniquad::conf::Conf {
            window_title: "RPG".to_string(),
            window_width: 960,
            window_height: 528,
            ..Default::default()
        },
        |_gctx| Box::new(App),
    );
}
