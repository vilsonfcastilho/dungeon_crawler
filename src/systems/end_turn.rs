use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state: TurnState = turn_state.clone();

    let mut new_state: TurnState = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MosterTurn,
        TurnState::MosterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|hp| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });

    *turn_state = new_state;
}
