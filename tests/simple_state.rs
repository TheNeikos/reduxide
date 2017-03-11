use std::default::Default;
#[macro_use] extern crate reduxide;

state! {
    State,
    [
        health: Health + HealthActions -> health_reducer,
        other_health: Health + HealthActions -> health_reducer
    ]
}

#[derive(Clone)]
struct Health {
    sick: bool,
    hp: u8,
}

impl Default for Health {
    fn default() -> Health {
        Health {
            sick: false,
            hp: 100,
        }
    }
}

enum HealthActions {
    Add(u8),
    Sub(u8),
    Set(u8),
    GetSick,
}

fn health_reducer(state: &mut Health, action: &HealthActions) {
    match *action {
        HealthActions::Add(amt) => state.hp += amt,
        HealthActions::Sub(amt) => state.hp -= amt,
        HealthActions::Set(amt) => state.hp = amt,
        HealthActions::GetSick => state.sick = true,
    }
}

#[test]
fn health() {
    let state = State::new();
    let new_state = state.reduce(&HealthActions::Set(200))
        .reduce(&HealthActions::Sub(10))
        .reduce(&HealthActions::Add(10));
    assert_eq!(new_state.health.hp, 200);
}

#[test]
fn other_health_also_updates() {
    let state = State::new();
    let new_state = state.reduce(&HealthActions::Set(200))
        .reduce(&HealthActions::Sub(10))
        .reduce(&HealthActions::Add(10));
    assert_eq!(new_state.other_health.hp, 200);
}

#[test]
fn get_sick() {
    let state = State::new();
    let new_state = state.reduce(&HealthActions::GetSick);
    assert_eq!(new_state.health.sick, true);
}
