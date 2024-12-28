use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GuiToEngine {
    Uci,
    UciNewGame,
    IsReady,
    Quit,
    Position(PositionType),
    Go,
}

pub enum EngineToGui {
    Id(IdType),
    UciOk,
    ReadyOk,
    Option(OptionType),
    Info(InfoType),
}

pub enum OptionType {
    Name(String),
}

pub enum IdType {
    Name(String),
    Author(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionType {
    FenString(Vec<String>),
    StartPos(Vec<String>),
}

pub enum InfoType {
    Str(String),
}

impl TryFrom<&str> for GuiToEngine {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let string_split: Vec<String> = value.split(" ").map(|str| str.to_string()).collect();
        match &string_split[..] {
            [string, ..] if string == "uci" => Ok(GuiToEngine::Uci),
            [string, ..] if string == "ucinewgame" => Ok(GuiToEngine::UciNewGame),
            [string, ..] if string == "isready" => Ok(GuiToEngine::IsReady),
            [string, ..] if string == "quit" => Ok(GuiToEngine::Quit),
            [string, ..] if string == "go" => Ok(GuiToEngine::Go),

            // TODO make the range not include the 'move' token. This should require an unstable feature? 1..
            [position, position_type, pos_vec @ ..] if position == "position" => {
                match position_type.as_str() {
                    "startpos" => Ok(GuiToEngine::Position(PositionType::StartPos(
                        pos_vec.to_vec(),
                    ))),
                    "fen" => Ok(GuiToEngine::Position(PositionType::FenString(
                        pos_vec.to_vec(),
                    ))),
                    _ => todo!("Unrecognized attribute {}", position_type),
                }
            }

            // if no string is provided
            _ => Err(format!("Invalid input {value}")),
        }
    }
}

impl Display for EngineToGui {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineToGui::UciOk => f.write_str("uciok"),
            EngineToGui::Id(kind) => match kind {
                IdType::Name(name) => f.write_fmt(format_args!("id name {}", name)),
                IdType::Author(author) => f.write_fmt(format_args!("id author {}", author)),
            },
            EngineToGui::Info(info) => match info {
                InfoType::Str(string) => f.write_fmt(format_args!("info string {}", string)),
            },
            EngineToGui::ReadyOk => f.write_str("readyok"),
            EngineToGui::Option(OptionType::Name(name)) => {
                f.write_fmt(format_args!("option name {}", name))
            }
        }
    }
}
