use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{default::Default, time::{SystemTime, UNIX_EPOCH}};

use super::utils;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcalidrawFile {
    pub r#type: String,
    pub version: u32,
    pub source: String,
    pub elements: Vec<Element>,
}

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
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Element {
    Rectangle(RectangleElement),
    Diamond(DiamondElement),
    Ellipse(EllipseElement),
    Arrow(ArrowElement),
    Line(LineElement),
    Freedraw(FreedrawElement),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseElement {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
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
    pub updated: i64,
    pub link: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roundness {
    pub r#type: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RectangleElement {
    #[serde(flatten)]
    pub base: BaseElement,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiamondElement {
    #[serde(flatten)]
    pub base: BaseElement,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EllipseElement {
    #[serde(flatten)]
    pub base: BaseElement,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrowElement {
    #[serde(flatten)]
    pub base: BaseElement,
    pub points: Vec<(f64, f64)>,
    pub last_committed_point: Option<serde_json::Value>,
    pub start_binding: Option<serde_json::Value>,
    pub end_binding: Option<serde_json::Value>,
    pub start_arrowhead: Option<String>,
    pub end_arrowhead: Option<String>,
    pub elbowed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineElement {
    #[serde(flatten)]
    pub base: BaseElement,
    pub points: Vec<(f64, f64)>,
    pub last_committed_point: Option<serde_json::Value>,
    pub start_binding: Option<serde_json::Value>,
    pub end_binding: Option<serde_json::Value>,
    pub start_arrowhead: Option<String>,
    pub end_arrowhead: Option<String>,
    pub polygon: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreedrawElement {
    #[serde(flatten)]
    pub base: BaseElement,
    pub points: Vec<(f64, f64)>,
}

impl Default for BaseElement {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            id: utils::rand_element_id(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            angle: 0.0,
            stroke_color: "#000000".to_string(),
            background_color: "transparent".to_string(),
            fill_style: "hachure".to_string(),
            stroke_width: 1,
            stroke_style: "solid".to_string(),
            roughness: 1,
            opacity: 100,
            group_ids: vec![],
            frame_id: None,
            index: "1".to_string(),
            roundness: Some(Roundness { r#type: 3 }),
            seed: 1,
            version: 12,
            version_nonce: rng.random_range(0..30000),
            is_deleted: false,
            bound_elements: None,
            updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
            link: None,
            locked: false,
        }
    }
}

impl Default for Roundness {
    fn default() -> Self {
        Self { r#type: 1 }
    }
}

impl Default for RectangleElement {
    fn default() -> Self {
        Self {
            base: BaseElement::default(),
        }
    }
}

impl RectangleElement {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        stroke_color: Option<String>,
        background_color: Option<String>,
    ) -> Self {
        Self {
            base: BaseElement {
                x,
                y,
                width,
                height,
                stroke_color: stroke_color.unwrap_or_else(|| "#000000".to_string()),
                background_color: background_color.unwrap_or_else(|| "#ffffff".to_string()),
                ..Default::default()
            },
        }
    }
}

impl Default for DiamondElement {
    fn default() -> Self {
        Self {
            base: BaseElement::default(),
        }
    }
}

impl DiamondElement {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            base: BaseElement {
                x,
                y,
                width,
                height,
                ..Default::default()
            },
        }
    }
}

impl Default for EllipseElement {
    fn default() -> Self {
        Self {
            base: BaseElement::default(),
        }
    }
}
impl EllipseElement {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            base: BaseElement {
                x,
                y,
                width,
                height,
                ..Default::default()
            },
        }
    }
}

impl Default for ArrowElement {
    fn default() -> Self {
        Self {
            base: BaseElement::default(),
            points: vec![(0.0, 0.0), (1.0, 1.0)],
            last_committed_point: None,
            start_binding: None,
            end_binding: None,
            start_arrowhead: Some("arrow".to_string()),
            end_arrowhead: Some("arrow".to_string()),
            elbowed: false,
        }
    }
}
impl ArrowElement {
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Self {
            base: BaseElement {
                x: start.0,
                y: start.1,
                width: end.0 - start.0,
                height: end.1 - start.1,
                ..Default::default()
            },
            points: vec![(0.0, 0.0), (end.0 - start.0, end.1 - start.1)],
            start_arrowhead: Some("dot".to_string()),
            end_arrowhead: Some("arrow".to_string()),
            ..Default::default()
        }
    }
}

impl Default for LineElement {
    fn default() -> Self {
        Self {
            base: BaseElement::default(),
            points: vec![(0.0, 0.0), (1.0, 1.0)],
            last_committed_point: None,
            start_binding: None,
            end_binding: None,
            start_arrowhead: None,
            end_arrowhead: None,
            polygon: false,
        }
    }
}
impl LineElement {
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        let (x, y) = points.get(0).copied().unwrap_or((100.0, 100.0));
        Self {
            base: BaseElement {
                x,
                y,
                width: 0.0,
                height: 0.0,
                ..Default::default()
            },
            points,
            ..Default::default()
        }
    }
}

impl Default for FreedrawElement {
    fn default() -> Self {
        Self {
            base: BaseElement::default(),
            points: vec![(0.0, 0.0), (1.0, 1.0)],
        }
    }
}

impl FreedrawElement {
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        let (x, y) = points.first().copied().unwrap_or((100.0, 100.0));
        Self {
            base: BaseElement {
                x,
                y,
                ..Default::default()
            },
            points,
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Element::Rectangle(RectangleElement::default())
    }
}

impl Element {
    pub fn new_rectangle() -> Self {
        Element::Rectangle(RectangleElement::default())
    }
}
