use std::marker::PhantomData;
use bevy::prelude::*;
use lunex_engine::*;

/// ## Compute camera ui
/// This function triggers computation method on camera's [`UiTree`] component if there was a change.
pub fn compute_camera_ui<T:Default + Component>(mut query: Query<(&Camera, &mut UiTree<T>), Or<(Changed<Camera>, Changed<UiTree<T>>)>>) {
    for (cam, mut ui) in &mut query {

        // Extract camera size
        if let Some(size) = cam.physical_viewport_size() {
            let size: (u32, u32) = size.into();

            // Compute the UI with the extracted size
            ui.compute(Rect2D::new().with_size((size.0 as f32, size.1 as f32)).into());
        }
    }
}

pub fn draw_debug_gizmo<T:Default + Component>(mut query: Query<(&UiTree<T>, &Transform)>, mut gizmos: Gizmos) {
    for (tree, transform) in &mut query {
        let list = tree.crawl();
        for node in list {
            if let Some(container) = node.obtain_data() {

                let mut color = Color::LIME_GREEN;

                if let Layout::Solid(_) = container.layout { color = Color::YELLOW }

                let mut pos = container.rect.pos.invert_y() + transform.translation;
                pos.x += container.rect.size.x / 2.0;
                pos.y += container.rect.size.y / -2.0;

                gizmos.rect(
                    pos,
                    Quat::from_rotation_y(0.0),
                    container.rect.size,
                    color,
                );
            }
        }
    }
}


//pub fn weird<T: Component>(mut query: Query<&mut T>) {}



/// ## UI plugin
/// THis
#[derive(Debug, Default, Clone)]
pub struct UiPlugin <T:Default + Component>(pub PhantomData<T>);
impl <T:Default + Component> UiPlugin<T> {
    pub fn new() -> Self {
        UiPlugin::<T>(PhantomData)
    }
}
impl <T:Default + Component> Plugin for UiPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_debug_gizmo::<T>)
            .add_systems(Update, compute_camera_ui::<T>);
    }
}