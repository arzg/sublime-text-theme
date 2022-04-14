use once_cell::sync::OnceCell;

static PALETTE: OnceCell<Palette> = OnceCell::new();

pub(crate) struct Palette {
    pub(crate) bg: u32,
    pub(crate) fg: u32,
    pub(crate) red: u32,
    pub(crate) orange: u32,
    pub(crate) yellow: u32,
    pub(crate) green: u32,
    pub(crate) blue: u32,
}

impl Palette {
    pub(crate) fn new() -> &'static Self {
        PALETTE.get_or_init(|| Palette {
            bg: 0x282923,
            fg: 0xf8f8f2,
            red: 0xf92472,
            orange: 0xfd9621,
            yellow: 0xe7db74,
            green: 0xa6e22c,
            blue: 0x67d8ef,
        })
    }
}
