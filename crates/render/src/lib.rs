//! Rendering layer.
//!
//! 2D canvas drawing, 3D skin rendering, and stats card compositions.

pub mod canvas;
pub mod cards;
pub mod icons;
pub mod skin;

// Re-export commonly used types
pub use canvas::{Canvas, Shape, init as init_canvas};
pub use cards::session::SessionType;
pub use cards::{TagIcon, render_bedwars, render_prestiges, render_session};
pub use skin::{OutputType, Pose, RenderOutput, Rotation, Skin, SkinError, render as render_skin};
