use raylib::{Color, Vector2};

const MAX_ITERATIONS: u64 = 170;
const SCREEN_W: i32 = 1280;
const SCREEN_H: i32 = 720;
const HALF_SCREEN_W: i32 = SCREEN_W/2;
const HALF_SCREEN_H: i32 = SCREEN_H/2;

#[inline]
fn julia(x: f64, y: f64, cx: f64, cy: f64) -> u64 {
   let mut zx = (x/SCREEN_W as f64) * 2.5;
   let mut zy = (y/SCREEN_H as f64) * 1.5;
   let mut iterations: u64 = 0;
   
   while zx * zx + zy * zy < 4.0 && iterations < MAX_ITERATIONS {
      let xtemp = zx * zx - zy * zy;
      zy = 2.0 * zx * zy + cy;
      zx = xtemp + cx;

      iterations += 1;
   }

   iterations
}

struct Pixel {
   x: i32,
   y: i32,
   col: Color,
}

fn run_julia(offset: Vector2, cx: f64, cy: f64) -> Vec<Pixel> {
   let mut out: Vec<Pixel> = vec![];

   for i in 0..SCREEN_W {
      for j in 0..SCREEN_H {
         let iter = julia(i as f64 + offset.x as f64, j as f64 + offset.y as f64, cx, cy);
         let norm_val: f64 = iter as f64/MAX_ITERATIONS as f64;
         let norm = (norm_val * 255.0).floor() as u8;

         out.push(Pixel { x: i, y: j, col: Color { r: norm, g: norm, b: norm, a: 255 }});
      }
   }

   out
}

fn main() {
   let rl = raylib::init()
      .size(SCREEN_W, SCREEN_H)
      .title("Julia")
      .build();
 
   rl.set_target_fps(60);

   let mut rendered = false;
   let mut cx: f64 = -0.8;
   let mut cy: f64 = 0.156;


   let mut pixels: Vec<Pixel> = vec![];
   while !rl.window_should_close() {
      let dt = rl.get_frame_time();
      cx += 0.005 * dt as f64;
      cy += 0.001 * dt as f64;
      rendered = false;

      if !rendered {
         pixels = run_julia(Vector2 { x: -HALF_SCREEN_W as f32, y: -HALF_SCREEN_H as f32 }, cx, cy);
         rendered = true;
      }

      rl.begin_drawing();
      rl.clear_background(Color::BLACK);
      for p in pixels.iter() {
         rl.draw_pixel(p.x, p.y, p.col);
      }

      rl.draw_fps(10, 10);
      rl.end_drawing();
   }
}
