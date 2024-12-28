use crate::board::BoardState;
use crate::uci::{EngineToGui, GuiToEngine, IdType, InfoType, OptionType, PositionType};
use std::fs::File;
use std::io::Write;

mod board;
mod uci;

fn main() {
    let mut debug_state = true;

    let mut log_file = File::options()
        .truncate(true)
        .write(true)
        .create(true)
        .open("/home/uraniumnnutt/Documents/programming/rust/nuttchess/log.txt")
        .unwrap();
    let mut board = BoardState::new();

    enum BotState {
        UCI,
        Starting,
        Running,
        Finished,
    }

    let mut current_state = BotState::UCI;

    loop {
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();
        log_file.write_all(input_string.as_bytes()).unwrap();
        log_file.flush().unwrap();

        if input_string == "\n" || input_string.len() == 0 {
            continue;
        }
        let gui_to_engine = GuiToEngine::try_from(input_string.as_str().trim());
        match current_state {
            BotState::UCI => match gui_to_engine {
                Ok(GuiToEngine::Uci) => {
                    log_file
                        .write_all(
                            EngineToGui::Id(IdType::Name("nuttchess".into()))
                                .to_string()
                                .as_bytes(),
                        )
                        .unwrap();
                    println!("{}", EngineToGui::Id(IdType::Name("nuttchess".into())));

                    log_file
                        .write_all(
                            EngineToGui::Id(IdType::Author("UraniumNutt".into()))
                                .to_string()
                                .as_bytes(),
                        )
                        .unwrap();
                    println!("{}", EngineToGui::Id(IdType::Author("UraniumNutt".into())));

                    log_file
                        .write_all(
                            EngineToGui::Option(OptionType::Name("Debug".into()))
                                .to_string()
                                .as_bytes(),
                        )
                        .unwrap();
                    println!("{}", EngineToGui::Option(OptionType::Name("Debug".into())));

                    log_file
                        .write_all(EngineToGui::UciOk.to_string().as_bytes())
                        .unwrap();

                    println!("{}", EngineToGui::UciOk);
                    current_state = BotState::Starting;
                }
                Err(string) => {
                    if debug_state {
                        log_file
                            .write_all(
                                EngineToGui::Info(InfoType::Str(format!(
                                    "Unexpected input {}",
                                    string
                                )))
                                .to_string()
                                .as_bytes(),
                            )
                            .unwrap();

                        println!(
                            "{}",
                            EngineToGui::Info(InfoType::Str(format!(
                                "Unexpected input {}",
                                string
                            )))
                        )
                    }
                }
                _ => continue,
            },
            BotState::Starting => match gui_to_engine {
                Ok(GuiToEngine::IsReady) => {
                    log_file
                        .write_all(EngineToGui::ReadyOk.to_string().as_bytes())
                        .unwrap();

                    println!("{}", EngineToGui::ReadyOk)
                }
                Ok(GuiToEngine::Quit) => break,

                Ok(GuiToEngine::UciNewGame) => {
                    current_state = BotState::Running;
                    continue;
                }
                Err(string) => {
                    if debug_state {
                        log_file
                            .write_all(
                                EngineToGui::Info(InfoType::Str(format!(
                                    "Unexpected input {}",
                                    string
                                )))
                                .to_string()
                                .as_bytes(),
                            )
                            .unwrap();
                        println!(
                            "{}",
                            EngineToGui::Info(InfoType::Str(format!(
                                "Unexpected input {}",
                                string
                            )))
                        )
                    }
                }
                _ => continue,
            },

            BotState::Running => match gui_to_engine {
                Ok(GuiToEngine::Position(position_type)) => match position_type {
                    PositionType::FenString(_) => {
                        board::board_state_from_pos(&position_type);
                    }
                    PositionType::StartPos(_) => {
                        board::board_state_from_pos(&position_type);
                    }
                },

                Ok(GuiToEngine::IsReady) => {
                    log_file
                        .write_all(EngineToGui::ReadyOk.to_string().as_bytes())
                        .unwrap();
                    println!("{}", EngineToGui::ReadyOk)
                }

                Ok(GuiToEngine::Go) => {
                    // println!(
                    //     "{}",
                    //     EngineToGui::Info(InfoType::Str(format!("{:?}", board)))
                    // );
                    // log::log();

                    log_file
                        .write_all("bestmove d7d5".to_string().as_bytes())
                        .unwrap();
                    println!("bestmove d7d5");
                }

                Ok(GuiToEngine::Quit) => {
                    // current_state = BotState::Finished;
                    // continue;
                    break;
                }
                Err(string) => {
                    if debug_state {
                        log_file
                            .write_all(
                                EngineToGui::Info(InfoType::Str(format!(
                                    "Unexpected input {}",
                                    string
                                )))
                                .to_string()
                                .as_bytes(),
                            )
                            .unwrap();
                        println!(
                            "{}",
                            EngineToGui::Info(InfoType::Str(format!(
                                "Unexpected input {}",
                                string
                            )))
                        )
                    }
                }
                _ => continue,
            },

            BotState::Finished => {
                break;
            }
        }
    }
}
