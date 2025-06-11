use rand::Rng;
use serde::{Deserialize, Serialize};
use std::
    default::Default
;

use super::utils::{self, rand_element_id, updated_timestamp};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Element {
    Rectangle(Rectangle),
    Arrow(ExcalidrawArrow),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcalidrawFile {
    pub r#type: String,
    pub version: u32,
    pub source: String,
    pub elements: Vec<Element>,
    pub app_state: AppState,
    // FIXME: this is wrong no string here but some object but will do for now
    pub files: Files,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Files {}

impl ExcalidrawFile {
    fn new(source: String, elements: Vec<Element>) -> ExcalidrawFile {
        ExcalidrawFile {
            source,
            elements,
            ..Default::default()
        }
    }
}

impl Default for ExcalidrawFile {
    fn default() -> Self {
        ExcalidrawFile {
            r#type: "excalidraw".to_string(),
            version: 2,
            source: "https://excalidraw.com".to_string(),
            elements: vec![],
            app_state: AppState::default(),
            files: Files {},
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rectangle {
    pub id: String,
    pub r#type: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: u64,
    pub stroke_color: String,
    pub background_color: String,
    pub fill_style: String,
    pub stroke_width: u32,
    pub stroke_style: String,
    pub roughness: u32,
    pub opacity: u32,
    pub group_ids: Vec<String>,
    pub frame_id: Option<String>,
    pub index: String,
    pub roundness: Option<Roundness>,
    pub seed: u64,
    pub version: u32,
    pub version_nonce: u64,
    pub is_deleted: bool,
    pub bound_elements: Option<serde_json::Value>,
    pub updated: u128,
    pub link: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roundness {
    pub r#type: u8,
}

impl Default for Rectangle {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            id: utils::rand_element_id(),
            r#type: "rectangle".to_string(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            angle: 0,
            stroke_color: "#000000".to_string(),
            background_color: "transparent".to_string(),
            fill_style: "solid".to_string(),
            stroke_width: 1,
            stroke_style: "solid".to_string(),
            roughness: 1,
            opacity: 100,
            group_ids: vec![],
            frame_id: None,
            index: "b01".to_string(),
            roundness: Some(Roundness { r#type: 3 }),
            seed: 1,
            version: 12,
            version_nonce: rng.random_range(0..u64::MAX),
            is_deleted: false,
            bound_elements: None,
            updated: updated_timestamp(),
            link: None,
            locked: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    grid_size: usize,
    grid_step: usize,
    grid_mode_enabled: bool,
    view_background_color: String,
    locked_multi_selections: LockedMultiSelections,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            grid_size: 20,
            grid_step: 5,
            grid_mode_enabled: false,
            view_background_color: "#ffffff".to_string(),
            locked_multi_selections: LockedMultiSelections {},
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockedMultiSelections {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcalidrawArrow {
    pub id: String,
    #[serde(rename = "type")]
    pub element_type: String, // e.g., "arrow"
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: u64,
    pub stroke_color: String,
    pub background_color: String,
    pub fill_style: String,
    pub stroke_width: u32,
    pub stroke_style: String,
    pub roughness: u32,
    pub opacity: u32,
    pub group_ids: Vec<String>,
    pub frame_id: Option<String>,
    pub index: String,
    pub roundness: Option<Roundness>,
    pub seed: i64,
    pub version: u32,
    pub version_nonce: i64,
    pub is_deleted: bool,
    pub bound_elements: Option<serde_json::Value>,
    pub updated: u128,
    pub link: Option<String>,
    pub locked: bool,
    pub points: Vec<[f64; 2]>,
    pub last_committed_point: Option<[f64; 2]>,
    pub start_binding: Option<Binding>,
    pub end_binding: Option<Binding>,
    pub start_arrowhead: Option<String>,
    pub end_arrowhead: Option<String>,
    pub elbowed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    pub element_id: String,
    pub focus: f64,
    pub gap: f64,
}

impl Default for ExcalidrawArrow {
    fn default() -> Self {
        Self {
            id: rand_element_id(),
            element_type: "arrow".to_string(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            angle: 0,
            stroke_color: "#000000".to_string(),
            background_color: "transparent".to_string(),
            fill_style: "solid".to_string(),
            stroke_width: 1,
            stroke_style: "solid".to_string(),
            roughness: 1,
            opacity: 100,
            group_ids: Vec::new(),
            frame_id: None,
            index: String::new(),
            roundness: None,
            seed: 0,
            version: 1,
            version_nonce: 0,
            is_deleted: false,
            bound_elements: None,
            updated: updated_timestamp(),
            link: None,
            locked: false,
            points: vec![[0.0, 0.0], [0.0, 0.0]],
            last_committed_point: None,
            start_binding: None,
            end_binding: None,
            start_arrowhead: None,
            end_arrowhead: Some("arrow".to_string()),
            elbowed: false,
        }
    }
}

enum Side {
    RIGHT,
    LEFT,
    TOP,
    BOTTOM,
}
