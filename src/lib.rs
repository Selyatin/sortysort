#[cfg(target_family = "wasm")]
use js_sys::Math;

#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};

use eframe::{egui::*, epi};

#[cfg(target_family = "wasm")]
fn get_randomized_f32_vec(max_size: f32, amount: usize) -> Vec<f32> {
    let mut vec = Vec::with_capacity(amount);
    for _ in 0..amount {
        let mut random = Math::random() as f32 * max_size;
        if random < 5.0 {
            random += 5.0 - random;
        }
        vec.push(random);
    }
    vec
}

#[cfg(not(target_family = "wasm"))]
fn get_randomized_f32_vec(max_size: f32, amount: usize) -> Vec<f32> {
    let mut rng = thread_rng();
    let mut vec = Vec::with_capacity(amount);
    for _ in 0..amount {
        vec.push(rng.gen_range(5..max_size as u32) as f32);
    }
    vec
}

pub struct App {
    lines: Vec<f32>,
    line_width: f32,
    lines_len: usize,
    index: usize,
    sorted: bool,
    sorting: bool,
    available_size: Vec2
}

impl Default for App {
    fn default() -> Self {
        Self {
            lines: vec![],
            line_width: 0.0,
            lines_len: 100,
            sorting: false,
            sorted: false,
            index: 1,
            available_size: Vec2::default()
        }
    }
}

impl App {
    fn insertion_sort(&mut self){
        if self.index == self.lines.len() {
            self.sorting = false;
            return;
        }
        
        let first = self.index - 1;
        let second = self.index;

        if self.lines[second] < self.lines[first] {
            let temp = self.lines[first];
            self.lines[first] = self.lines[second];
            self.lines[second] = temp;
            if self.index != 1 {
                self.index = self.index.wrapping_sub(1);
            }
            return;
        }
        self.index += 1;
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Sortysort"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(f32::INFINITY, f32::INFINITY)
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            
            if available_size != self.available_size || self.lines_len != self.lines.len() {
                self.lines = get_randomized_f32_vec(available_size.y, self.lines_len);
                self.line_width = available_size.x / self.lines_len as f32;
                self.available_size = available_size;
            }

            let (mut painter_response, painter) = ui.allocate_painter(
                available_size,
                Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );

            if self.sorting {
                self.insertion_sort();
                ctx.request_repaint(); 
            }

            let width_with_gap = self.line_width - 2.0;

            for (i, y) in self.lines.iter().enumerate() {
                let x = i as f32 * self.line_width;
                let color = if self.index == i {
                    Color32::RED
                } else {
                    Color32::LIGHT_BLUE
                };
                painter.add(Shape::line_segment(
                    [Pos2::new(x, 0.0), Pos2::new(x, *y)],
                    Stroke::new(width_with_gap, color),
                ));
            }
        });
         
        containers::panel::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Sort").clicked() {
                    self.sorting = true;
                    self.index = 1;
                }
                if ui.button("Randomize").clicked() {
                    self.available_size = Vec2::default();
                }
                ui.add(Slider::new(&mut self.lines_len, 10..=1000)
                    .logarithmic(true)
                    .text("Lines"));
            });
        });
    }
}

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_family = "wasm")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = App::default();
    eframe::start_web(canvas_id, Box::new(app))
}
