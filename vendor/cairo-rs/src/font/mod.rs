// Take a look at the license at the top of the repository in the LICENSE file.

mod font_extents;
mod font_face;
mod font_options;
mod glyph;
mod scaled_font;
mod text_cluster;
mod text_extents;
mod user_fonts;

pub use crate::enums::{
    Antialias, FontSlant, FontType, FontWeight, HintMetrics, HintStyle, SubpixelOrder,
    TextClusterFlags,
};

/* TODO
 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.


impl Glyph {

    //pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *Glyph;

    //pub fn cairo_glyph_free(glyphs: *Glyph);
}

 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.

impl TextCluster {
    //pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *TextCluster;

    //pub fn cairo_text_cluster_free(clusters: *TextCluster);
}
*/

pub use self::font_extents::FontExtents;
pub use self::font_face::FontFace;
pub use self::font_options::FontOptions;
pub use self::glyph::Glyph;
pub use self::scaled_font::ScaledFont;
pub use self::text_cluster::TextCluster;
pub use self::text_extents::TextExtents;
pub use self::user_fonts::UserFontFace;
