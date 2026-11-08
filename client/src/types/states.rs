#[derive(PartialEq)]
pub enum State {
    EnterPseudo,
    EnterPassword,
    Home,
    Saves,
    Replay,
    ChooseLobby,
    CreateLobby,
    InLobby,
    InGame,
    ModifyParameters,
}

#[derive(PartialEq)]
pub enum Focus {
    Game,
    Chat,
}

#[derive(PartialEq)]
pub enum GameState {
    WaitInit,
    WaitPlay,
    WaitChoosePiece,
    WaitEnd,
}
