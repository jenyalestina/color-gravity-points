use ggez::*;
use rand::prelude::*;
use ggez::graphics::BLACK;
use hsluv::hsluv_to_rgb;

fn main() {
    let mut rng = rand::thread_rng();
    let state = &mut State {
        dt: std::time::Duration::new(0,0),
        points: (0..1000).map(|x| Point {
            x: rng.gen_range(0f32,200f32),
            y: rng.gen_range(0f32,200f32),
            dx: rng.gen_range(-100f32,100f32),
            dy: rng.gen_range(-100f32,100f32),
        }).collect()
    };
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

struct State {
    dt: std::time::Duration,
    points: Vec<Point>,
}

fn color_husl(h: f64, s: f64, l: f64, a: f32) -> graphics::Color {
    let (r64,g64,b64) = hsluv_to_rgb((h,s,l));
    let r = r64 as f32;
    let g = g64 as f32;
    let b = b64 as f32;
    graphics::Color {r,g,b,a}
}

impl ggez::event::EventHandler for State {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);
        for point in &self.points {
            let sum = point.dx+point.dy;
            let hue = sum / 6.6666666;
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                mint::Point2{x:point.x,y:point.y},
                1.0,
                0.1,
                color_husl(hue.into(),100.0,50.0,1.0)
            )?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        //makes points bounce off the walls
        self.dt = timer::delta(ctx);
        let delta = self.dt.as_secs_f32();
        let screen = graphics::screen_coordinates(ctx);
        let mut dx_arr = vec![];
        let mut dy_arr = vec![];
        for point in &mut self.points {
            point.x += delta*point.dx;
            point.y += delta*point.dy;
            if point.x > screen.w || point.x < 0.0 {
                point.dx = -point.dx;
            }
            if point.y > screen.h || point.y < 0.0 {
                point.dy = -point.dy;
            }
            dx_arr.push(point.dx);
            dy_arr.push(point.dy);
        }

        //makes points attracted to each other
        for other_point_idx in 0..self.points.len() {
            for point_idx in 0..self.points.len() {
                if point_idx == other_point_idx {
                    continue;
                }
                let other_point_ox = self.points[other_point_idx].x - self.points[point_idx].x;
                let other_point_oy = self.points[other_point_idx].y - self.points[point_idx].y;
                let distance = (other_point_ox.powf(2.0)+other_point_oy.powf(2.0)).sqrt();
                let influence = distance.sqrt();
                let norm_x = other_point_ox/distance;
                let norm_y = other_point_oy/distance;
                self.points[point_idx].dx += norm_x / influence;
                self.points[point_idx].dy += norm_y / influence;
            }
        }
        Ok(())
    }
}
