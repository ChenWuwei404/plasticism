//! [`Atom`]s are compositions of a [`Widget`](crate::widget::Widget).

use std::sync::{Arc, Mutex};

use grafo::{DrawCommandError, Renderer};

use crate::core::{measurement::{Rect, Space}, utils::IdAllocator};

/// Compositions of a [`Widget`](crate::widget::Widget), which should fill up whole widget.
/// 
/// These trait defines `min_space` and `draw` functions. For properties controlling,
/// see specific structs.
/// 
/// A simple example is [`Rectangle`].
pub trait Atom: 'static {
    /// How much space a atom needs to display.
    fn min_space(&self) -> Space {
        Space::new(0.0, 0.0)
    }

    /// Queue draw command of current atom and get the id of the shape.
    /// 
    /// Note that shape id is **not** atom id.
    fn draw(
        &self,
        rect: &Rect,
        renderer: &mut Renderer,
        parent_shape_id: Option<usize>,
        scale_factor: f64
    ) -> Result<usize, DrawCommandError>;
}

/// Due to a limitation that textures must be numbered in [`grafo`], an `u64` typed id must
/// be given when creating a atom include texture. See [`IdAllocator`]. AtomWithTexture-structs
/// should implement custom [`Drop`] to automatically deallocate id. Just invoke 
/// [`AtomWithTexture::deallocate`] in [`Drop::drop`] with [`auto_drop`].
pub trait AtomWithTexture: Atom {
    /// Get a new id from given allocator (singleton).
    fn new(id_allocator: Arc<Mutex<IdAllocator>>) -> Self where Self: Sized;
    
    fn deallocate(&self) {
        self.id_allocator().lock().unwrap().deallocate(self.id());
    }

    fn id(&self) -> u64;
    fn id_allocator(&self) -> &Arc<Mutex<IdAllocator>>;
}

/// Basic struct for [`AtomWithTexture`] structs to store id information.
pub struct AtomId {
    pub id: u64,
    pub id_allocator: Arc<Mutex<IdAllocator>>,
}

#[macro_export]
macro_rules! auto_drop {
    ($id: ident) => {
        impl Drop for $id {
            /// Automatically deallocate id.
            fn drop(&mut self) {
                self.deallocate();
            }
        }
    };
}

/// Generate [`AtomWithTexture::id`] and [`AtomWithTexture::id_allocator`] for atoms with `atom_id:`[`AtomId`] field.
/// 
/// # Example
/// 
/// ```no_run
/// struct CustomAtom {
///     atom_id: AtomId,
///     /* other fields */
/// }
/// impl AtomWithTexture for CustomAtom {
///     auto_atom_id!();
///     /* other functions */
/// }
/// 
/// ```
#[macro_export]
macro_rules! auto_atom_id {
    () => {
        fn id(&self) -> u64 {
            self.atom_id.id
        }
        fn id_allocator(&self) -> &Arc<Mutex<IdAllocator>> {
            &self.atom_id.id_allocator
        }
    };
}

mod rectangle;
pub use rectangle::*;

mod label;
pub use label::*;
