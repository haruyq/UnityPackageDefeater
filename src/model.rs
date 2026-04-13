#[derive(Default)]
pub struct GuidEntry {
    pub pathname:   Option<String>,
    pub asset:      Option<Vec<u8>>,
    pub asset_meta: Option<Vec<u8>>,
}