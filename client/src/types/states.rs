#[derive(PartialEq)]
pub enum State {
    EnterPseudo,
    EnterPassword,
    Home,
    ChooseLobby,
    CreateLobby,
    InLobby,
    InGame,
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
