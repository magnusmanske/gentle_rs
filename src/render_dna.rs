use crate::{
    dna_sequence::DNAsequence, render_dna_circular::RenderDnaCircular,
    render_dna_linear::RenderDnaLinear,
};
use eframe::egui::{self, PointerState, Response, Sense, Ui, Widget};
use gb_io::seq::Feature;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub enum RenderDna {
    Circular(RenderDnaCircular),
    Linear(RenderDnaLinear),
}

impl RenderDna {
    pub fn new(dna: Arc<RwLock<DNAsequence>>) -> Self {
        let is_circular = dna.read().unwrap().is_circular();
        match is_circular {
            true => RenderDna::Circular(RenderDnaCircular::new(dna)),
            false => RenderDna::Linear(RenderDnaLinear::new(dna)),
        }
    }

    pub fn area(&self) -> &egui::Rect {
        match self {
            RenderDna::Circular(renderer) => renderer.area(),
            RenderDna::Linear(renderer) => renderer.area(),
        }
    }

    pub fn is_circular(&self) -> bool {
        match self {
            RenderDna::Circular(_) => true,
            RenderDna::Linear(_) => false,
        }
    }

    pub fn on_click(&mut self, pointer_state: PointerState) {
        match self {
            RenderDna::Circular(renderer) => renderer.on_click(pointer_state),
            RenderDna::Linear(renderer) => renderer.on_click(pointer_state),
        }
    }

    pub fn get_selected_feature_id(&self) -> Option<usize> {
        match self {
            RenderDna::Circular(renderer) => renderer.selected_feature_number(),
            RenderDna::Linear(renderer) => renderer.selected_feature_number(),
        }
    }

    pub fn select_feature(&mut self, feature_number: Option<usize>) {
        match self {
            RenderDna::Circular(renderer) => renderer.select_feature(feature_number),
            RenderDna::Linear(renderer) => renderer.select_feature(feature_number),
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        match self {
            RenderDna::Circular(renderer) => renderer.render(ui),
            RenderDna::Linear(renderer) => renderer.render(ui),
        }
    }

    pub fn feature_name(feature: &Feature) -> String {
        let mut label_text = match feature.location.find_bounds() {
            Ok((from, to)) => format!("{from}..{to}"),
            Err(_) => String::new(),
        };
        for k in [
            "name",
            "standard_name",
            "gene",
            "protein_id",
            "product",
            "region_name",
            "bound_moiety",
        ] {
            label_text = match feature.qualifier_values(k.into()).next() {
                Some(s) => s.to_owned(),
                None => continue,
            };
            break;
        }
        label_text
    }
}

impl Widget for RenderDna {
    fn ui(mut self, ui: &mut Ui) -> Response {
        self.render(ui);
        let response = ui.allocate_response(self.area().size(), Sense::click());
        response
        // Response::new(self.area().clone())
    }
}