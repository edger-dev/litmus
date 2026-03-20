use litmus_model::{Color, Theme};

pub struct ThemeWithExtras {
    pub theme: Theme,
    pub cursor: Color,
    pub selection: Color,
}

pub fn tokyo_night() -> ThemeWithExtras {
    ThemeWithExtras {
        theme: Theme {
            name: "Tokyo Night".to_string(),
            background: Color::new(0x1a, 0x1b, 0x26),
            foreground: Color::new(0xa9, 0xb1, 0xd6),
            colors: vec![
                // Normal (0-7)
                Color::new(0x15, 0x16, 0x22), // black
                Color::new(0xf7, 0x76, 0x8e), // red
                Color::new(0x9e, 0xce, 0x6a), // green
                Color::new(0xe0, 0xaf, 0x68), // yellow
                Color::new(0x7a, 0xa2, 0xf7), // blue
                Color::new(0xbb, 0x9a, 0xf7), // magenta
                Color::new(0x7d, 0xcf, 0xff), // cyan
                Color::new(0xa9, 0xb1, 0xd6), // white
                // Bright (8-15)
                Color::new(0x41, 0x44, 0x68), // bright black
                Color::new(0xf7, 0x76, 0x8e), // bright red
                Color::new(0x9e, 0xce, 0x6a), // bright green
                Color::new(0xe0, 0xaf, 0x68), // bright yellow
                Color::new(0x7a, 0xa2, 0xf7), // bright blue
                Color::new(0xbb, 0x9a, 0xf7), // bright magenta
                Color::new(0x7d, 0xcf, 0xff), // bright cyan
                Color::new(0xc0, 0xca, 0xf5), // bright white
            ],
        },
        cursor: Color::new(0xc0, 0xca, 0xf5),
        selection: Color::new(0x28, 0x3d, 0x5c),
    }
}

pub fn catppuccin_mocha() -> ThemeWithExtras {
    ThemeWithExtras {
        theme: Theme {
            name: "Catppuccin Mocha".to_string(),
            background: Color::new(0x1e, 0x1e, 0x2e),
            foreground: Color::new(0xcd, 0xd6, 0xf4),
            colors: vec![
                // Normal (0-7)
                Color::new(0x45, 0x47, 0x5a), // black
                Color::new(0xf3, 0x8b, 0xa8), // red
                Color::new(0xa6, 0xe3, 0xa1), // green
                Color::new(0xf9, 0xe2, 0xaf), // yellow
                Color::new(0x89, 0xb4, 0xfa), // blue
                Color::new(0xcb, 0xa6, 0xf7), // magenta
                Color::new(0x89, 0xdc, 0xeb), // cyan
                Color::new(0xba, 0xc2, 0xde), // white
                // Bright (8-15)
                Color::new(0x58, 0x5b, 0x70), // bright black
                Color::new(0xf3, 0x8b, 0xa8), // bright red
                Color::new(0xa6, 0xe3, 0xa1), // bright green
                Color::new(0xf9, 0xe2, 0xaf), // bright yellow
                Color::new(0x89, 0xb4, 0xfa), // bright blue
                Color::new(0xcb, 0xa6, 0xf7), // bright magenta
                Color::new(0x89, 0xdc, 0xeb), // bright cyan
                Color::new(0xcd, 0xd6, 0xf4), // bright white
            ],
        },
        cursor: Color::new(0xf5, 0xc2, 0xe7),
        selection: Color::new(0x31, 0x32, 0x44),
    }
}

pub fn solarized_dark() -> ThemeWithExtras {
    ThemeWithExtras {
        theme: Theme {
            name: "Solarized Dark".to_string(),
            background: Color::new(0x00, 0x2b, 0x36),
            foreground: Color::new(0x83, 0x94, 0x96),
            colors: vec![
                // Normal (0-7)
                Color::new(0x07, 0x36, 0x42), // black
                Color::new(0xdc, 0x32, 0x2f), // red
                Color::new(0x85, 0x99, 0x00), // green
                Color::new(0xb5, 0x89, 0x00), // yellow
                Color::new(0x26, 0x8b, 0xd2), // blue
                Color::new(0xd3, 0x36, 0x82), // magenta
                Color::new(0x2a, 0xa1, 0x98), // cyan
                Color::new(0xee, 0xe8, 0xd5), // white
                // Bright (8-15)
                Color::new(0x00, 0x2b, 0x36), // bright black
                Color::new(0xcb, 0x4b, 0x16), // bright red
                Color::new(0x58, 0x6e, 0x75), // bright green
                Color::new(0x65, 0x7b, 0x83), // bright yellow
                Color::new(0x83, 0x94, 0x96), // bright blue
                Color::new(0x6c, 0x71, 0xc4), // bright magenta
                Color::new(0x93, 0xa1, 0xa1), // bright cyan
                Color::new(0xfd, 0xf6, 0xe3), // bright white
            ],
        },
        cursor: Color::new(0x83, 0x94, 0x96),
        selection: Color::new(0x07, 0x36, 0x42),
    }
}

pub fn all_themes() -> Vec<ThemeWithExtras> {
    vec![tokyo_night(), catppuccin_mocha(), solarized_dark()]
}
