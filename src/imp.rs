use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule("editorCursor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("editorCursor.foreground", palette.base(BaseScale::BrightFg));

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rules(
        &[
            "editor.selectionBackground",
            "selection.background",
            "minimap.selectionHighlight",
        ],
        palette.teal(),
    );

    builder.add_workspace_rule(
        "editorLineNumber.foreground",
        palette.base(BaseScale::DarkerFg),
    );
    builder.add_workspace_rule(
        "editorLineNumber.activeForeground",
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rules(
        &["activityBar.background", "sideBar.background"],
        palette.base(BaseScale::DarkBg),
    );
    builder.add_workspace_rule("sideBar.foreground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("activityBar.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "activityBar.inactiveForeground",
        palette.base(BaseScale::DarkerFg),
    );

    builder.add_workspace_rules(
        &[
            "statusBar.background",
            "statusBar.noFolderBackground",
            "statusBar.debuggingBackground",
        ],
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rules(
        &["statusBar.foreground", "statusBar.noFolderForeground"],
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rules(
        &[
            "list.activeSelectionBackground",
            "list.inactiveSelectionBackground",
            "list.focusBackground",
            "list.hoverBackground",
        ],
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule(
        "list.activeSelectionForeground",
        palette.base(BaseScale::Fg),
    );
    builder.add_workspace_rule(
        "list.highlightForeground",
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rule("editorWidget.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("editorWidget.border", palette.base(BaseScale::DarkestBg));

    builder.add_workspace_rule("editor.findMatchHighlightBackground", palette.teal());
    builder.add_workspace_rule(
        "editor.findMatchBackground",
        palette.base(BaseScale::LighterBg),
    );
    builder.add_workspace_rules(
        &[
            "editor.findMatchBorder",
            "editorOverviewRuler.findMatchForeground",
            "minimap.findMatchHighlight",
        ],
        palette.yellow(),
    );

    builder.add_workspace_rule("input.background", palette.base(BaseScale::DarkestBg));

    builder.add_workspace_rules(
        &[
            "editorGroupHeader.tabsBackground",
            "tab.inactiveBackground",
            "tab.border",
        ],
        palette.base(BaseScale::DarkerBg),
    );
    builder.add_workspace_rule("tab.activeForeground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("tab.inactiveForeground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("editorGroup.border", palette.base(BaseScale::DarkestBg));

    builder.add_workspace_rule(
        "scrollbarSlider.background",
        (palette.base(BaseScale::LighterBg), 0x66),
    );
    builder.add_workspace_rules(
        &[
            "scrollbarSlider.hoverBackground",
            "scrollbarSlider.activeBackground",
        ],
        (palette.base(BaseScale::LighterBg), 0xAA),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.border",
        palette.base(BaseScale::LighterBg),
    );

    builder.add_workspace_rule("editorLightBulb.foreground", palette.yellow());

    builder.add_workspace_rule(
        "editorCodeLens.foreground",
        palette.base(BaseScale::DarkerFg),
    );

    builder.add_workspace_rule("focusBorder", palette.base(BaseScale::LighterBg));

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkerFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.green());
    builder.add_rule(Semantic("keyword.controlFlow"), palette.blue());
    builder.add_rules(
        &[
            Semantic("arithmetic"),
            Semantic("bitwise"),
            Semantic("comparison"),
            Semantic("logical"),
        ],
        palette.light_yellow(),
    );

    builder.add_rules(&[Semantic("function"), Semantic("method")], palette.cream());

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("interface"),
            Semantic("union"),
            Semantic("builtinType"),
            Semantic("typeParameter"),
        ],
        palette.orange(),
    );

    builder.add_rule(Semantic("number"), palette.light_orange());
    builder.add_rules(&[Semantic("string"), Semantic("character")], palette.cyan());

    builder.add_rules(
        &[
            Semantic("enumMember"),
            Semantic("variable.constant"),
            Semantic("variable.static"),
            Semantic("boolean"),
        ],
        palette.strong_cyan(),
    );

    builder.add_rule(Semantic("property"), palette.pink());

    builder.add_rules(
        &[Semantic("macro"), Semantic("*.attribute")],
        palette.dark_yellow(),
    );

    builder.add_rule(
        Semantic("comment"),
        (palette.base(BaseScale::BrightFg), FontStyle::Italic),
    );

    builder.add_rule(Semantic("*.mutable"), FontStyle::Underline);
    builder.add_rule(Semantic("*.public.declaration"), FontStyle::Bold);
}
