#[cfg(target_family = "wasm")]
use js_sys::Math;

#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};

use eframe::{egui::*, epi};
use std::collections::VecDeque;

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

#[derive(Debug, Eq, PartialEq)]
enum Algorithm {
    Quick,
    Insertion,
}

pub struct App {
    lines: Vec<f32>,
    line_width: f32,
    lines_len: usize,
    index: usize,
    // (low, high, pivot)
    sub_ranges: VecDeque<(usize, usize, usize, usize)>,
    sorting: bool,
    available_size: Vec2,
    speed: usize,
    algorithm: Algorithm,
}

impl Default for App {
    fn default() -> Self {
        Self {
            lines: vec![],
            line_width: 0.0,
            lines_len: 100,
            sorting: false,
            index: 1,
            sub_ranges: VecDeque::new(),
            available_size: Vec2::default(),
            speed: 1,
            algorithm: Algorithm::Insertion,
        }
    }
}

impl App {
    fn insertion_sort(&mut self) {
        for _ in 0..self.speed {
            if self.index == self.lines.len() {
                self.sorting = false;
                return;
            }

            let first = self.index - 1;
            let second = self.index;

            if self.lines[second] < self.lines[first] {
                self.lines.swap(first, second);
                if self.index != 1 {
                    self.index = self.index.wrapping_sub(1);
                }
                continue;
            }
            self.index += 1;
        }
    }

    fn quick_sort(&mut self) {
        for _ in 0..self.speed {
            let (low, high, i, j) = match self.sub_ranges.get(0) {
                Some(tuple) => *tuple,
                None => {
                    return self.sorting = false;
                }
            };

            let pivot = self.lines[high];

            if low >= high {
                self.sub_ranges.pop_front();
                continue;
            }

            if j < high {
                if self.lines[j] <= pivot {
                    self.lines.swap(i, j);
                    self.sub_ranges[0].2 += 1;
                }
                self.sub_ranges[0].3 += 1;
                continue;
            }

            self.lines.swap(i, high);
            self.sub_ranges.pop_front();

            if i > 0 {
                self.sub_ranges.push_front((low, i - 1, low, low));
            }

            if i < high {
                self.sub_ranges.push_back((i + 1, high, i + 1, i + 1));
            }
        }
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
                self.sub_ranges.clear();
                match self.algorithm {
                    Algorithm::Quick => {
                        self.sub_ranges.push_back((0, self.lines.len() - 1, 0, 0));
                    }
                    Algorithm::Insertion => {
                        self.index = 1;
                    }
                };
            }

            let (_painter_response, painter) = ui.allocate_painter(
                available_size,
                Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );

            if self.sorting {
                match self.algorithm {
                    Algorithm::Quick => self.quick_sort(),
                    Algorithm::Insertion => self.insertion_sort(),
                };
                ctx.request_repaint();
            }

            let width_with_gap = self.line_width - 2.0;

            for (i, y) in self.lines.iter().enumerate() {
                let x = i as f32 * self.line_width;

                let mut color = Color32::LIGHT_BLUE;

                match self.algorithm {
                    Algorithm::Quick => {
                        if let Some((_, _, n, j)) = self.sub_ranges.get(0) {
                            if i == *n || i == *j {
                                color = Color32::RED;
                            }
                        }
                    }
                    Algorithm::Insertion => {
                        if self.index == i {
                            color = Color32::RED
                        }
                    }
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
                    self.sub_ranges.push_back((0, self.lines.len() - 1, 0, 0));
                }
                if ui.button("Randomize").clicked() {
                    self.available_size = Vec2::default();
                }
                ui.add(
                    Slider::new(&mut self.lines_len, 10..=1000)
                        .logarithmic(true)
                        .text("Lines"),
                );
                ui.add(
                    Slider::new(&mut self.speed, 1..=20000)
                        .logarithmic(true)
                        .text("Speed"),
                );
                ui.radio_value(&mut self.algorithm, Algorithm::Insertion, "Insertion Sort");
                ui.radio_value(&mut self.algorithm, Algorithm::Quick, "Quick Sort");
                ui.label("Algorithm");
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
