use raylib::{consts, Color, Vector2};

const MAX_ITERATIONS: u64 = 512;
const SCREEN_W: i32 = 1920;
const SCREEN_H: i32 = 1080;
const HALF_SCREEN_W: i32 = SCREEN_W/2;
const HALF_SCREEN_H: i32 = SCREEN_H/2;

// Shader uniform locations
const SHADER_SCREEN_DIMS_LOC: i32 = 0;
const SHADER_MAX_ITERATIONS_LOC: i32 = 1;
const SHADER_C_LOC: i32 = 2;
const SHADER_OFFSET_LOC: i32 = 3;
const SHADER_ZOOM_LOC: i32 = 4;
const SHADER_TIME_LOC: i32 = 5;

const MOUSE_SCROLL_SPEED: f64 = 0.01;
const AUTO_SPEED: f64 = 0.005;



fn main() {
   let rl = raylib::init()
      .size(SCREEN_W, SCREEN_H)
      .title("Julia")
      .build();

   rl.set_target_fps(144);
   
   let points_of_interest: Vec<[f64; 2]> = vec![
      [-0.8, 0.156],
      [0.285, 0.0],
      [0.285, 0.01],
      [-0.835, -0.2321]
   ];
 
   let mut cx: f64 = points_of_interest[0][0];
   let mut cy: f64 = points_of_interest[0][1];

   let offset = Vector2 { x: -HALF_SCREEN_W as f32, y: -HALF_SCREEN_H as f32 };
   let zoom: f64 = 2.0;
   let mut forward = false; // Slowly increase c stuff
   let mut backward = false; // Slowly decrease c stuff

   let mut shader = rl.load_shader("", "src/julia_shader.fs");
   rl.set_shader_value(&mut shader, SHADER_SCREEN_DIMS_LOC, &[SCREEN_W as f32, SCREEN_H as f32]);
   rl.set_shader_value_i(&mut shader, SHADER_MAX_ITERATIONS_LOC, &[MAX_ITERATIONS as i32]);
   rl.set_shader_value(&mut shader, SHADER_C_LOC, &[cx as f32, cy as f32]);
   rl.set_shader_value(&mut shader, SHADER_OFFSET_LOC, &[offset.x, offset.y]);
   rl.set_shader_value(&mut shader, SHADER_ZOOM_LOC, &[zoom as f32]);

   while !rl.window_should_close() {
      if rl.is_key_pressed(consts::KEY_LEFT as i32) { // Pressing left goes back, but if going back and left is pressed, stop altogether.
         if backward {
            backward = false;
         } else {
            backward = true;
            forward = false;
         }
      } else if rl.is_key_pressed(consts::KEY_RIGHT as i32) {
         if forward {
            forward = false;
         } else {
            forward = true;
            backward = false;
         }
      }

      let mouse_mv = rl.get_mouse_wheel_move();
      if mouse_mv.abs() > 0 {
         if forward { forward = false };
         if backward { backward = false };
         let mut amount = MOUSE_SCROLL_SPEED * mouse_mv as f64;
         if rl.is_key_down(consts::KEY_LEFT_SHIFT as i32) {
            amount = amount/10.0;
         }

         cx += amount;
         cy += amount;
         rl.set_shader_value(&mut shader, SHADER_C_LOC, &[cx as f32, cy as f32]);
      }

      if forward || backward {
         let dt = rl.get_frame_time();
         //rl.set_shader_value(&mut shader, SHADER_TIME_LOC, &[rl.get_time()]);
         let amount = AUTO_SPEED * dt as f64;
         if forward {
            if backward { backward = false };
            cx += amount;
            cy += amount;
         } else if backward {
            cx -= amount;
            cy -= amount;
         }
         rl.set_shader_value(&mut shader, SHADER_C_LOC, &[cx as f32, cy as f32]);
      }

      rl.begin_drawing();
      rl.clear_background(Color::BLACK);

      rl.begin_shader_mode(&shader);
      rl.draw_rectangle(0, 0, SCREEN_W, SCREEN_H, Color::BLACK);
      rl.end_shader_mode();

      rl.draw_fps(10, SCREEN_H - 25);
      rl.draw_text(format!("frame time: {:.5}\ncx: {:.5}\ncy: {:.5}", rl.get_frame_time(), cx, cy).as_str(), 10, 10, 20, Color::RAYWHITE);
      rl.end_drawing();
   }
}
