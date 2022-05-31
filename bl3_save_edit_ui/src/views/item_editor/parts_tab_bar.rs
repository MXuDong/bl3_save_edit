use strum::Display;

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum AvailablePartType {
    #[strum(to_string = "可选面板")]
    Parts,
    #[strum(to_string = "可选受福")]
    Anointments,
}

impl std::default::Default for AvailablePartType {
    fn default() -> Self {
        Self::Parts
    }
}

#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum CurrentPartType {
    #[strum(to_string = "当前面板")]
    Parts,
    #[strum(to_string = "当前受福")]
    Anointments,
}

impl std::default::Default for CurrentPartType {
    fn default() -> Self {
        Self::Parts
    }
}
