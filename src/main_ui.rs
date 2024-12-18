use egui::Color32;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
enum NodeType {
    Coordiantor,
    Router,
    Endpoint,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Node {
    name: String,
    id: usize,
    position: egui::Pos2,
    node_type: NodeType,
    range: u32,
}

pub struct NodeApp {
    nodes: Vec<Node>,
    next_id: usize,
    dragging: Option<usize>,
    offset: egui::Vec2,
    selected: Option<usize>,
    save_path: String,
    load_path: String,
}

impl Default for NodeApp {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 0,
            dragging: None,
            offset: egui::Vec2::ZERO,
            selected: None,
            save_path: String::new(),
            load_path: String::new(),
        }
    }
}

impl NodeApp {
    fn save_nodes(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(&self.nodes)?;
        fs::write(&self.save_path, json)
    }

    fn load_nodes(&mut self) -> Result<(), std::io::Error> {
        if let Ok(json) = fs::read_to_string(&self.load_path) {
            if let Ok(loaded_nodes) = serde_json::from_str(&json) {
                self.nodes = loaded_nodes;
                // Recompute next_id to avoid conflicts
                self.next_id = self.nodes.iter().map(|n| n.id).max().unwrap_or(0) + 1;
            }
        }
        Ok(())
    }
}

impl eframe::App for NodeApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("save_load_panel").show(ctx, |ui| {
            ui.heading("Save/Load Nodes");

            ui.label("Save Path:");
            ui.text_edit_singleline(&mut self.save_path);
            if ui.button("Save Nodes").clicked() {
                if let Err(e) = self.save_nodes() {
                    eprintln!("Error saving nodes: {}", e);
                }
            }

            ui.separator();

            ui.label("Load Path:");
            ui.text_edit_singleline(&mut self.load_path);
            if ui.button("Load Nodes").clicked() {
                if let Err(e) = self.load_nodes() {
                    eprintln!("Error loading nodes: {}", e);
                }
            }


            ui.separator();
            // Düğüm seçimi
            if self.selected.is_some() {
                let selected_node = self
                    .nodes
                    .iter_mut()
                    .find(|n| n.id == self.selected.unwrap())
                    .unwrap();
                ui.label("Node Info");
                ui.label("Name:");
                ui.text_edit_singleline(&mut selected_node.name);
                ui.label("Type:");
                ui.label("Range:");
                ui.add(egui::Slider::new(&mut selected_node.range, 50..=200));

                ui.radio_value(
                    &mut selected_node.node_type,
                    NodeType::Coordiantor,
                    "Coordiantor",
                );
                ui.radio_value(&mut selected_node.node_type, NodeType::Router, "Router");
                ui.radio_value(&mut selected_node.node_type, NodeType::Endpoint, "Endpoint");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Fare konumunu al
            let mouse_pos = ui
                .input(|i| i.pointer.interact_pos())
                .unwrap_or(egui::Pos2::ZERO);

            // Sağ tıklama ile yeni düğüm oluştur
            if ui.input(|i| i.pointer.secondary_clicked()) {
                let mut rng = rand::thread_rng();
                let random_number = rng.gen_range(50..=200);
                let new_node = Node {
                    name: "Node".to_string(),
                    id: self.next_id,
                    position: mouse_pos,
                    node_type: NodeType::Endpoint,
                    range: random_number,
                };
                self.nodes.push(new_node);
                self.next_id += 1;
            }
            for node in &self.nodes {
                ui.painter().circle(
                    node.position,
                    node.range as f32,
                    Color32::from_rgba_premultiplied(255, 255, 0xE0, 230),
                    egui::Stroke::NONE,
                );
            }

            // Düğümler arasındaki bağlantıları çiz
            for node_m in &self.nodes {
                for node_i in &self.nodes {
                    if node_m.id != node_i.id {
                        if node_m.node_type == NodeType::Endpoint
                            && node_i.node_type == NodeType::Endpoint
                        {
                            continue;
                        } else if node_m.position.distance_sq(node_i.position)
                            > f32::powi((node_m.range + node_i.range) as f32, 2)
                        {
                            continue;
                        }
                        ui.painter().line_segment(
                            [node_m.position, node_i.position],
                            egui::Stroke::new(3.0, Color32::RED),
                        );
                    }
                }
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
                        self.selected = Some(node.id);
                    }
                }

                if node_response.drag_stopped() {
                    self.dragging = None;
                }

                // Düğüm renigini türe göre seç
                let node_color = match node.node_type {
                    NodeType::Coordiantor => Color32::LIGHT_RED,
                    NodeType::Router => Color32::LIGHT_GREEN,
                    NodeType::Endpoint => Color32::LIGHT_BLUE,
                };

                ui.painter().rect_filled(node_rect, 5.0, node_color);
                ui.painter().text(
                    node_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("{}\n{}", node.id, node.name),
                    egui::FontId::proportional(13.0),
                    Color32::BLACK,
                );
            }
        });
    }
}
