#[derive(PartialEq)]
pub enum State {
    EnterPseudo,
    Home,
    ChooseLobby,
    CreateLobby,
    InLobby,
    InGame,
}
