use std::fmt::{Display, format, Formatter};

#[derive(Debug)]
pub enum GuiToEngine {
    Uci,
    UciNewGame,
    IsReady,
    Quit,
    Position(PositionType),

}

pub enum EngineToGui {
    Id(IdType),
    UciOk,
    ReadyOk,
}

pub enum IdType {
    Name(String),
    Author(String),
}

#[derive(Debug)]
pub enum PositionType {
    FenString(String),
    StartPos,
}

impl TryFrom<&str> for GuiToEngine {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut string_split = value.split(" ");
        match (string_split.next().or(Some(value))) {

            (Some(string)) if string == "uci" => Ok(GuiToEngine::Uci),
            (Some(string)) if string == "ucinewgame" => Ok(GuiToEngine::UciNewGame),
            (Some(string)) if string == "isready" => Ok(GuiToEngine::IsReady),
            (Some(string)) if string == "quit" => Ok(GuiToEngine::Quit),



            // if no string is provided
            _ => Err(format!("Invalid input {value}"))
        }
    }

}

impl Display for EngineToGui {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                EngineToGui::UciOk => f.write_str("uciok"),
                EngineToGui::Id(kind) => {
                    match kind {
                        IdType::Name(name) => f.write_fmt(format_args!("id name {}", name)),
                        IdType::Author(author) => f.write_fmt(format_args!("id author {}", author)),
                    }
                }
                EngineToGui::ReadyOk => f.write_str("readyok"),
            }
    }
}