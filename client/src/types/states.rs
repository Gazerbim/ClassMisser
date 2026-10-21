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
pub enum GameState {
    WaitInit,
}
