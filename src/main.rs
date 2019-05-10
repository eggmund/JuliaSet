use raylib::{Color, Vector2, RaylibHandle};

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

/*
struct Pixel {
   x: i32,
   y: i32,
   col: Color,
}
*/

#[inline]
fn get_colour(norm: f64) -> Color {
   let scaled = (norm * 255.0).floor() as u8;
   Color { r: scaled, g: scaled/3, b: (scaled/2) + 127, a: 255}
}

fn run_julia(rl: &RaylibHandle, offset: Vector2, zoom: f64, cx: f64, cy: f64) {
   for i in 0..SCREEN_W {
      for j in 0..SCREEN_H {
         let iter = julia((i as f64 + offset.x as f64) * zoom, (j as f64 + offset.y as f64) * zoom, cx, cy);
         let norm: f64 = iter as f64/MAX_ITERATIONS as f64;

         rl.draw_pixel(i, j, get_colour(norm.powf(1.0/3.0)));
      }
   }
}

fn main() {
   let rl = raylib::init()
      .size(SCREEN_W, SCREEN_H)
      .title("Julia")
      .build();
 
   rl.set_target_fps(60);

   let mut cx: f64 = -0.77322;
   let mut cy: f64 = 0.15868;
   let zoom: f64 = 1.3;


   //let mut pixels: Vec<Pixel> = vec![];

   while !rl.window_should_close() {
      let dt = rl.get_frame_time();
      cx += -0.001 * dt as f64;
      cy += -0.0001 * dt as f64;


      rl.begin_drawing();
      rl.clear_background(Color::BLACK);

      run_julia(&rl, Vector2 { x: -HALF_SCREEN_W as f32, y: -HALF_SCREEN_H as f32 }, zoom, cx, cy);

      rl.draw_text(format!("frame time: {:.5}\ncx: {:.5}\ncy: {:.5}", rl.get_frame_time(), cx, cy).as_str(), 10, 10, 20, Color::RAYWHITE);
      rl.end_drawing();
   }
}
