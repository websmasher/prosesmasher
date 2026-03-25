/// Supported document locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    #[default]
    En,
    Ru,
    De,
    Es,
    Pt,
    Fr,
    Id,
}
