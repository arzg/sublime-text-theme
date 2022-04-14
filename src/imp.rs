use crate::palette::Palette;
use mottle::dsl::{s, FontStyle, ThemeBuilder};

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

    t.a([s("parameter.declaration"), s("selfKeyword")], (p.orange, FontStyle::Italic));

    t.a([s("string")], p.yellow);

    t.a(
        [
            s("function.declaration"),
            s("method.declaration"),
            s("type.declaration"),
            s("class.declaration"),
            s("struct.declaration"),
            s("enum.declaration"),
            s("interface.declaration"),
            s("union.declaration"),
            s("typeAlias.declaration"),
            s("typeParameter.declaration"),
            s("namespace.declaration"),
        ],
        p.green,
    );
}
