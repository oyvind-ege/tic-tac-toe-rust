use crate::GameState;

pub enum InputType {
    Coord(usize),
    Exit,
    Restart,
    Help,
}

pub trait InputController {
    fn get_raw_input(&self) -> String;
    fn parse_input(&self, input: &str) -> Option<InputType>;
}

pub trait PlayerController {
    fn handle_input(&self, gamestate: &GameState) -> Option<InputType>;
}
