use macroquad::{color::Color, prelude::{BLACK, WHITE}};

pub const INVERT_COLORS: bool = false;

/** usually used as background */
pub const PRIMARY_COLOR: Color = if INVERT_COLORS { BLACK } else { WHITE };
/** usually used as foreground */
pub const SECONDARY_COLOR: Color = if INVERT_COLORS { WHITE } else { BLACK };