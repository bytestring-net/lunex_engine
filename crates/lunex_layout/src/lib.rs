use lunex_core::NodeSize;

pub mod prelude {
    pub use super::lui;
}

/// # Lunex UI
/// Contains all containers available in
/// ### Declarative
/// * [Window]
/// * [Solid]
/// ### Parametric
/// * [Div]
/// * [List]
/// * [Grid]
pub mod lui {
    pub use crate::declarative::*;
    pub use crate::parametric::*;
}





pub struct TextCapsule {
    pub align_x: Align, // Style
    pub align_y: Align, // Style

    pub text: String, // text
    pub wrap: bool,   // Auto wrap lines at width
}



/// # Align
/// Type used for aligning items in parametric containers.
/// 
/// _Range_ : `-1.0 for START to 1.0 for END`
/// * `Align::START`
/// * `Align::CENTER`
/// * `Align::END`
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Align (pub f32);
impl Align {
    pub const START: Align = Align(-1.0);
    pub const LEFT: Align = Align(-1.0);
    pub const CENTER: Align = Align(0.0);
    pub const MIDDLE: Align = Align(0.0);
    pub const END: Align = Align(1.0);
    pub const RIGHT: Align = Align(1.0);
}


pub struct Container {
    layout: Layout,
    text: Option<TextCapsule>, // It modifies ContentSize though?

    depth: f32,

    roll: f32,
    yaw: f32,
    pitch: f32
}


pub enum Layout {
    Window(declarative::Window),
    Solid(declarative::Solid),
    //Window3D
    //Div
    //Br
}



/// ## Div Size
/// 
/// * [DivSize::Min]
/// * [DivSize::Max]
/// * [DivSize::Exact]
pub enum DivSize<T> {
    /// ## Min
    /// Div will be as little as it can be while still encapsulating it's content and following set rules.
    Min,
    /// ## Max
    /// Div will be as big as it can be while following set rules. If `max_width` is not set then fill `100%` of the parent layout.
    Max,
    /// ## Exact
    /// Div will be sized as specified while still following set rules.
    Exact(NodeSize<T>),
}

/// ## Declarative Layouts
/// Contains declarative type of layouts.
/// You define their exact position. They don't rely on context.
/// They are the primitives of this library.
/// * [Window]
/// * [Solid]
pub mod declarative {
    use bevy::math::Vec2;
    use lunex_core::{Prc, NodeSize};
    use crate::Align;
    use crate::Layout;

    /// ## Window Layout
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    pub struct Window {
        /// ## Position
        /// Position of the top-left corner.
        pub pos : NodeSize<Vec2>,
        /// ## Size
        /// Size of the layout.
        pub size: NodeSize<Vec2>,
    }
    impl Window {
        /// ## Full Window
        /// Covers 100% of the parent layout.
        pub const FULL: Window = Window { pos : NodeSize::from_prc(Vec2::ZERO), size: NodeSize::from_prc(Vec2::splat(100.0)) };
        /// ## New
        /// Creates new Window layout.
        pub const fn new() -> Self {
            Window {
                pos : NodeSize::from_prc(Vec2::ZERO),
                size: NodeSize::from_prc(Vec2::ZERO),
            }
        }
        /// ## With pos
        /// Replaces the position with the new value.
        pub fn with_pos(mut self, pos: NodeSize<Vec2>) -> Self {
            self.pos = pos;
            self
        }
        /// ## With x
        /// Replaces the x position with the new value.
        pub fn with_x(mut self, width: NodeSize<f32>) -> Self {
            self.pos.set_x(width);
            self
        }
        /// ## With y
        /// Replaces the y position with the new value.
        pub fn with_y(mut self, height: NodeSize<f32>) -> Self {
            self.pos.set_y(height);
            self
        }
        /// ## With size
        /// Replaces the size with the new value.
        pub fn with_size(mut self, size: NodeSize<Vec2>) -> Self {
            self.size = size;
            self
        }
        /// ## With width
        /// Replaces the width with the new value.
        pub fn with_width(mut self, width: NodeSize<f32>) -> Self {
            self.size.set_x(width);
            self
        }
        /// ## With height
        /// Replaces the height with the new value.
        pub fn with_height(mut self, height: NodeSize<f32>) -> Self {
            self.size.set_y(height);
            self
        }
    }
    impl Into<Layout> for Window {
        fn into(self) -> Layout {
            Layout::Window(self)
        }
    }

    /// ## Solid Layout
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    pub struct Solid {
        /// ## Size
        /// Aspect ratio of the sides of the rectangular layout. `1:1 == 10:10 == 100:100`.
        pub size: NodeSize<Vec2>,
        /// ## Align X
        /// Horizontal alignment within parent.
        pub align_x: Align,
        /// ## Align Y
        /// Vertical alignment within parent.
        pub align_y: Align
    }
    impl Solid {
        /// ## New
        /// Creates new Solid layout.
        pub fn new() -> Self {
            Solid {
                size: Prc(Vec2::ONE).into(),
                align_x: Align::CENTER,
                align_y: Align::CENTER,
            }
        }
        /// ## With size
        /// Replaces the size with the new value.
        pub fn with_size(mut self, size: NodeSize<Vec2>) -> Self {
            self.size = size;
            self
        }
        /// ## With width
        /// Replaces the width with the new value.
        pub fn with_width(mut self, width: NodeSize<f32>) -> Self {
            self.size.set_x(width);
            self
        }
        /// ## With height
        /// Replaces the height with the new value.
        pub fn with_height(mut self, height: NodeSize<f32>) -> Self {
            self.size.set_y(height);
            self
        }
        /// ## With align x
        /// Replaces the x alignment with the new value.
        pub fn with_align_x(mut self, align: Align) -> Self {
            self.align_x = align;
            self
        }
        /// ## With align y
        /// Replaces the y alignment with the new value.
        pub fn with_align_y(mut self, align: Align) -> Self {
            self.align_y = align;
            self
        }
    }
    impl Into<Layout> for Solid {
        fn into(self) -> Layout {
            Layout::Solid(self)
        }
    }
}



/// ## Parametric Layouts
/// Contains parametric type of layouts.
/// You define how they behave based on their neighboring nodes.
/// They rely on context. They work similarly as HTML (padding, margin, etc.)
/// * [Div]
/// * [List]
/// * [Grid]
pub mod parametric {
    use bevy::math::Vec4;
    //use crate::Align;
    use crate::DivSize;
    use lunex_core::{Prc, NodeSize};

    pub struct Div { // Most basic type, basically every div is List 
        pub width: DivSize<f32>,
        pub min_width: Option<NodeSize<f32>>,
        pub max_width: Option<NodeSize<f32>>,

        pub height: DivSize<f32>,
        pub min_height: Option<NodeSize<f32>>,
        pub max_height: Option<NodeSize<f32>>,

        pub padding: NodeSize<Vec4>,
        pub margin: NodeSize<Vec4>,
    }

    pub struct Br; // Just div with new line class
    pub struct List; //Ver or Hor (Flex) or Chain?
    pub struct Grid;    // (Grid)
}