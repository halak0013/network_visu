use eframe::egui;
use egui::Pos2;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    position: egui::Pos2,
    selected: bool,
}

struct NodeApp {
    nodes: Vec<Node>,
    next_id: usize,
    dragging: Option<usize>,
    offset: egui::Vec2,
}

impl Default for NodeApp {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 0,
            dragging: None,
            offset: egui::Vec2::ZERO,
        }
    }
}

impl eframe::App for NodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Fare konumunu al
            let mouse_pos = ui
                .input(|i| i.pointer.interact_pos())
                .unwrap_or(egui::Pos2::ZERO);

            // Sağ tıklama ile yeni düğüm oluştur
            if ui.input(|i| i.pointer.secondary_clicked()) {
                let new_node = Node {
                    id: self.next_id,
                    position: mouse_pos,
                    selected: false,
                };
                self.nodes.push(new_node);
                self.next_id += 1;
            }

            // Düğümleri çiz ve etkileşimi yönet
            for node in &mut self.nodes {
                let node_rect =
                    egui::Rect::from_center_size(node.position, egui::Vec2::new(50.0, 50.0));

                // Fare ile sürükleme
                let node_response = ui.allocate_rect(node_rect, egui::Sense::drag());

                if node_response.drag_started() {
                    self.dragging = Some(node.id);
                    self.offset = node.position.to_vec2() - mouse_pos.to_vec2();
                }

                if let Some(dragged_id) = self.dragging {
                    if dragged_id == node.id {
                        node.position = (mouse_pos.to_vec2() + self.offset).to_pos2();
                    }
                }

                if node_response.drag_released() {
                    self.dragging = None;
                }

                // Düğümleri çiz
                ui.painter()
                    .rect_filled(node_rect, 5.0, egui::Color32::LIGHT_BLUE);
                ui.painter().text(
                    node_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    node.id.to_string(),
                    egui::FontId::proportional(15.0),
                    egui::Color32::BLACK,
                );
                ui.painter().line_segment(
                    [Pos2::new(71.0, 51.1), Pos2::new(22.0, 2.1)],
                    egui::Stroke::new(3.0, egui::Color32::RED),
                );
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        //initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Node Hareket Uygulaması",
        options,
        Box::new(|_cc| Box::<NodeApp>::default()),
    )
}
