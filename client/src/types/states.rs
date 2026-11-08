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
}

#[derive(PartialEq)]
pub enum Focus {
    Game,
    Chat,
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    WaitInit,
    WaitPlay,
    WaitChoosePiece,
    WaitEnd,
    APlayerWon,
}
