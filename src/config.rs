use macroquad::{color::Color, prelude::{BLACK, WHITE}};

pub const INVERT_COLORS: bool = false;

/** usually used as background */
pub const PRIMARY_COLOR: Color = if INVERT_COLORS { BLACK } else { WHITE };
/** usually used as foreground */
pub const SECONDARY_COLOR: Color = if INVERT_COLORS { WHITE } else { BLACK };

pub const TEXTURE_SCALING: f32 = 4.0;
pub const SHOUT_TEX_SCALE: f32 = 7.0;