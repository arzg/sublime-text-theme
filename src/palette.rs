use once_cell::sync::OnceCell;

static PALETTE: OnceCell<Palette> = OnceCell::new();

pub(crate) struct Palette {
    pub(crate) bg: u32,
    pub(crate) fg: u32,
}

impl Palette {
    pub(crate) fn new() -> &'static Self {
        PALETTE.get_or_init(|| Palette { bg: 0x000000, fg: 0xFFFFFF })
    }
}
