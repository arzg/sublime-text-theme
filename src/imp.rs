use crate::palette::Palette;
use mottle::dsl::{s, ThemeBuilder};

pub(crate) fn add_rules(t: &mut ThemeBuilder, p: &Palette) {
    workspace_colors(t, p);
    syntax_highlighting(t, p);
}

fn workspace_colors(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["editor.background"], p.bg);
    t.w(["editor.foreground", "foreground"], p.fg);
}

fn syntax_highlighting(t: &mut ThemeBuilder, p: &Palette) {
    t.a([s("keyword"), s("operator")], p.red);
}
