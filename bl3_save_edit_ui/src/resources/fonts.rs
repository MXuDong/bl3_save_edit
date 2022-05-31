use iced::Font;

pub const JETBRAINS_MONO: Font = Font::External {
    name: "Jetbrains Mono",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Regular.ttf"),
};

pub const ST_HEI_TI_LIGHT_BOLD: Font = Font::External {
    name: "Jetbrains Mono Bold",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Bold.ttf"),
};

pub const ST_HEI_TI_LIGHT_NL_EXTRA_BOLD_ITALIC: Font = Font::External {
    name: "Jetbrains Mono NL Extra Bold Italic",
    bytes: include_bytes!("../../resources/font/JetBrainsMonoNL-ExtraBoldItalic.ttf"),
};

pub const ST_HEI_TI_LIGHT_LIGHT_ITALIC: Font = Font::External {
    name: "Jetbrains Mono Light Italic",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-LightItalic.ttf"),
};

pub const ST_HEI_TI_LIGHT: Font = Font::External {
    name: "STHeiti Light",
    bytes: include_bytes!("../../resources/font/STHeiti Light.ttc")
};