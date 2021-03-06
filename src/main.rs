extern crate maat_graphics;
extern crate maat_input_handler;
extern crate rand;
extern crate csv;
extern crate hlua;
extern crate open;

use hlua::Lua;
use maat_graphics::imgui::*;
pub use maat_graphics::winit;
pub use maat_graphics::cgmath;

mod modules;

use crate::modules::scenes::Scene;
use crate::modules::scenes::LoadScreen;

use maat_graphics::graphics::CoreRender;
use maat_graphics::CoreMaat;
use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector4};

use std::time;

const MAJOR: u32 = 0;
const MINOR: u32 = 2;
const PATCH: u32 = 1;

fn benchmark(draw_calls: &mut Vec<DrawCall>, dimensions: Vector2<f32>) {
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(dimensions.x - 80.0, 15.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "v".to_string() + &MAJOR.to_string() + "." + &MINOR.to_string() + "." + &PATCH.to_string(), 
                                           "Arial".to_string()));
}

fn fps_overlay(draw_calls: &mut Vec<DrawCall>, dimensions: Vector2<f32>, fps: f64) {
  let  mut fps = fps.to_string();
  fps.truncate(6);
  
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(32.0, dimensions.y-48.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                           "fps: ".to_string() + &fps, 
                                           "Arial".to_string()));
}

fn main() {
  let mut lua = Lua::new();
  
  let mut imgui = ImGui::init();
  let mut graphics = CoreMaat::new("Maat Editor".to_string(), (MAJOR) << 22 | (MINOR) << 12 | (PATCH), 1280.0, 1080.0, true).use_imgui(&mut imgui);
  
  graphics.preload_font(String::from("Arial"),
                        String::from("./resources/Fonts/TimesNewRoman.png"),
                        include_bytes!("../resources/Fonts/TimesNewRoman.fnt"));
  graphics.preload_texture(String::from("Logo"), 
                           String::from("./resources/Textures/Logo.png"));
  
  graphics.add_model(String::from("Axis"), String::from("./Models/Axis.glb"));
  
  graphics.load_shaders();
  
  graphics.set_clear_colour(0.2, 0.2, 0.2, 1.0);
  
  let mut game: Box<Scene> = Box::new(LoadScreen::new());
  
  let mut draw_calls: Vec<DrawCall> = Vec::with_capacity(100);
  
  let mut delta_time;
  let mut last_time = time::Instant::now();
  let mut imgui_time = time::Instant::now();
  
  let mut done = false;
  let mut dimensions;
  
  let mut frame_counter = 0;
  let mut fps_timer = 0.0;
  let mut last_fps = 0.0;
  
  loop {
    delta_time = last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
    last_time = time::Instant::now();
    
    frame_counter += 1;
    fps_timer += delta_time;
    if fps_timer > 1.0 {
      last_fps = frame_counter as f64 * (1.0/fps_timer);
      fps_timer = 0.0;
      frame_counter = 0;
    }
    
    dimensions = graphics.get_virtual_dimensions();
    
    let models = game.get_models_to_unload();
    for reference in &models {
      draw_calls.push(DrawCall::unload_model(reference.to_string()));
    }
    
    let models = game.get_models_to_load();
    for (reference, location) in &models {
      graphics.add_model(reference.to_string(), location.to_string());
      draw_calls.push(DrawCall::load_model(reference.to_string()));
    }
    
    if game.scene_finished() {
      game = game.future_scene(dimensions);
    }
    
    game.set_window_dimensions(dimensions);
    
    graphics.prepare_imgui_ui(Some(&mut imgui));
    imgui_time = imgui.io_mut().update_delta_time(imgui_time);
    let ui = imgui.frame();
    
    game.draw(&mut draw_calls);
    game.update(Some(&ui),  Some(&mut lua), delta_time as f32);
    
    benchmark(&mut draw_calls, dimensions);
    fps_overlay(&mut draw_calls, dimensions, last_fps);
    
    let model_details = graphics.pre_draw();
    graphics.draw(&draw_calls, Some(ui), delta_time as f32);
    graphics.post_draw();
    
    draw_calls.clear();
    
    game.reset_scroll_value();
    for (reference, size) in &model_details {
      game.add_model_size(reference.to_string(), *size);
    }
    
    let events = graphics.get_events(Some(&mut imgui));
    let mouse_pos = graphics.get_mouse_position();
    
    game.set_mouse_position(mouse_pos);
    
    for ev in events {
      match &ev {
        winit::Event::WindowEvent{ event, .. } => {
          match event {
            winit::WindowEvent::CloseRequested => {
              done = true;
            },
            _ => {
              if game.handle_input(event) {
                done = true;
              }
            }
          }
        },
        _ => {},
      }
    }
    
    if done { break; }
  }
  
  println!("Game Loop ended");
}
