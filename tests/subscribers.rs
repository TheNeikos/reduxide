use std::default::Default;
#[macro_use] extern crate reduxide;

state! {
    State,
    [
        health: Health > HealthActions > health_reducer
    ]
}

#[derive(Debug, Clone, PartialEq)]
struct Health {
    hp: u8,
}

impl Default for Health {
    fn default() -> Health {
        Health {
            hp: 100,
        }
    }
}

enum HealthActions {
    Add(u8),
    Sub(u8),
    Set(u8),
}

fn health_reducer(state: &mut Health, action: &HealthActions) {
    match *action {
        HealthActions::Add(amt) => state.hp += amt,
        HealthActions::Sub(amt) => state.hp -= amt,
        HealthActions::Set(amt) => state.hp = amt,
    }
}

#[test]
fn subscribers() {
    let mut state = State::new();
    let sub = state.subscribe();
    let state = state.reduce(&HealthActions::Set(200))
        .reduce(&HealthActions::Sub(10))
        .reduce(&HealthActions::Add(10));
    assert_eq!(state.health.hp, 200);
    let _ = sub.recv().unwrap();
    let _ = sub.recv().unwrap();
    let rec_state = sub.recv().unwrap();
    assert_eq!(rec_state.health, state.health);
}

