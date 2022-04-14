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
    t.a([s("keyword"), s("arithmetic"), s("logical"), s("bitwise"), s("comparison")], p.red);

    t.a(
        [s("parameter.declaration"), s("selfKeyword"), s("typeParameter")],
        (p.orange, FontStyle::Italic),
    );

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
            s("namespace.declaration"),
        ],
        p.green,
    );

    t.a(
        [
            s("type"),
            s("class"),
            s("struct"),
            s("enum"),
            s("interface"),
            s("union"),
            s("typeAlias"),
            s("builtinType"),
        ],
        (p.blue, FontStyle::Italic),
    );

    t.a([s("function"), s("method")], p.blue);
}
