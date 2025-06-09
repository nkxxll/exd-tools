use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    time::{SystemTime, UNIX_EPOCH},
};

use super::utils;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcalidrawFile {
    pub r#type: String,
    pub version: u32,
    pub source: String,
    pub elements: Vec<Rectangle>,
    pub app_state: AppState,
    // FIXME: this is wrong no string here but some object but will do for now
    pub files: Files,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Files {}

impl ExcalidrawFile {
    fn new(source: String, elements: Vec<Rectangle>) -> ExcalidrawFile {
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
            updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
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
