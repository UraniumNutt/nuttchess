use crate::uci::{EngineToGui, GuiToEngine, IdType};

mod uci;
mod board;

fn main() {

    loop {

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();

        if input_string == "\n" || input_string.len() == 0 {
            continue;
        }
        let gui_to_engine = GuiToEngine::try_from(input_string.as_str().trim()).unwrap();
        match gui_to_engine {
            GuiToEngine::Uci => {
                println!("{}", EngineToGui::Id(IdType::Name("nuttchess".into())));
                println!("{}", EngineToGui::Id(IdType::Author("UraniumNutt".into())));;
                println!("{}", EngineToGui::UciOk);
            }
            GuiToEngine::IsReady => println!("{}", EngineToGui::ReadyOk),
            GuiToEngine::Quit => break,
            _ => todo!()
        }


    }


}