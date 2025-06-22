//! This is the connector that connects the DSL to the excalidraw library.
//! For this the From trait is implemented for the DSL types.
//! The connector is a simple wrapper around the excalidraw creation tools.

use crate::excalidraw::structures::ExcalidrawFile;

use super::parser::Tree;

impl From<Tree> for ExcalidrawFile {
    fn from(value: Tree) -> Self {
        todo!()
    }
}
