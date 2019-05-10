use raylib::{Color, Vector2, RaylibHandle};

const MAX_ITERATIONS: u64 = 200;
const SCREEN_W: i32 = 1280;
const SCREEN_H: i32 = 720;
const HALF_SCREEN_W: i32 = SCREEN_W/2;
const HALF_SCREEN_H: i32 = SCREEN_H/2;

const SHADER_SCREEN_DIMS_LOC: i32 = 0;
const SHADER_MAX_ITERATIONS_LOC: i32 = 1;
const SHADER_C_LOC: i32 = 2;
const SHADER_OFFSET_LOC: i32 = 3;
const SHADER_ZOOM_LOC: i32 = 4;


fn main() {
   let rl = raylib::init()
      .size(SCREEN_W, SCREEN_H)
      .title("Julia")
      .build();
 
   let mut cx: f64 = -0.77322;
   let mut cy: f64 = 0.15868;
   let offset = Vector2 { x: -HALF_SCREEN_W as f32, y: -HALF_SCREEN_H as f32 };
   let zoom: f64 = 1.3;

   let mut shader = rl.load_shader("", "src/julia_shader.fs");
   rl.set_shader_value(&mut shader, SHADER_SCREEN_DIMS_LOC, &[SCREEN_W as f32, SCREEN_H as f32]);
   rl.set_shader_value_i(&mut shader, SHADER_MAX_ITERATIONS_LOC, &[MAX_ITERATIONS as i32]);
   rl.set_shader_value(&mut shader, SHADER_C_LOC, &[cx as f32, cy as f32]);
   rl.set_shader_value(&mut shader, SHADER_OFFSET_LOC, &[offset.x, offset.y]);
   rl.set_shader_value(&mut shader, SHADER_ZOOM_LOC, &[zoom as f32]);

   while !rl.window_should_close() {
      let dt = rl.get_frame_time();
      cx += -0.001 * dt as f64;
      cy += -0.0001 * dt as f64;
      rl.set_shader_value(&mut shader, SHADER_C_LOC, &[cx as f32, cy as f32]);


      rl.begin_drawing();
      rl.clear_background(Color::BLACK);

      //run_julia(&rl, Vector2 { x: -HALF_SCREEN_W as f32, y: -HALF_SCREEN_H as f32 }, zoom, cx, cy);
      rl.begin_shader_mode(&shader);
      rl.draw_rectangle(0, 0, SCREEN_W, SCREEN_H, Color::BLACK);
      rl.end_shader_mode();

      rl.draw_fps(10, SCREEN_H - 25);
      rl.draw_text(format!("frame time: {:.5}\ncx: {:.5}\ncy: {:.5}", rl.get_frame_time(), cx, cy).as_str(), 10, 10, 20, Color::RAYWHITE);
      rl.end_drawing();
   }
}
