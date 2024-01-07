use crate::{Rect3D, NiceDisplay};
use colored::Colorize;

use crate::nodes::prelude::*;
use crate::layout::Layout;


pub type UINodeTree<P = ()> = NodeTree<InterfaceData, Container<P>>;
pub type UINode<P = ()> = Node<Container<P>>;


pub struct InterfaceData {
    //pub themes: Theme,
}

/// ## Container
/// A struct holding all UI data appended to [`UINode`]. Responsible for storing layout, custom data, cache, etc.
/// Every [`UINode`] needs to have this to work properly.
#[derive(Debug, Default)]
pub struct Container<P> {
    pub data: Option<P>,
    pub rect: Rect3D,
    pub layout: Layout,
    //text: Option<TextCapsule>, // It modifies ContentSize though?

    //depth: f32,
}

impl <P:Default> Container<P> {
    pub fn new() -> Container<P> {
        Container::default()
    }
}

impl <P> NiceDisplay for Container<P> {
    fn to_nicestr(&self) -> String {
        format!("{} {} {}", self.layout.to_nicestr(), "|||".black(), self.rect.to_nicestr())
    }
}