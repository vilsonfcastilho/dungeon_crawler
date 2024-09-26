#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MosterTurn,
    GameOver,
    Victory,
    NextLevel,
}
