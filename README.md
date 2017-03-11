# Reduxide

Reduxide is a heavily [redux][redux] inspired library.
It's basic premise is state management through a specific data flow.
To achieve this a 'data pipeline' is constructed from side-effect free functions
that are chained together. A side-effect free function is one that does not
modify any outside state and always gives the same output for the same input.

A high-level example:

```rust
fn handle_health(state: &mut Health, action: HealthActions) {
    let state = state.clone();
    match action {
        HealthActions::Add(amt) => state.hp += amt,
        HealthActions::Remove(amt) => state.hp -= amt,
        HealthActions::Set(amt) => state.hp = amt,
    }
}
```



