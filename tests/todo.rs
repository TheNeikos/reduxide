use std::default::Default;
#[macro_use] extern crate reduxide;

state! {
    TodoState,
    [
        todos: Vec<Todo> > TodoAction > todo_reducer
    ]
}

#[derive(Clone)]
struct Todo {
    text: String,
    completed: bool,
    id: u64,
}

impl Default for Todo {
    fn default() -> Todo {
        Todo {
            text: String::new(),
            completed: false,
            id: 0,
        }
    }
}

enum TodoAction {
    Add(String),
    Delete(u64),
    Edit(u64, String),
    Complete(u64),
    CompleteAll,
    ClearCompleted,
}

fn todo_reducer(state: &mut Vec<Todo>, action: &TodoAction) {
    use TodoAction::*;
    let max_id = state.iter().fold(0u64, |acc, x| if acc > x.id { acc } else { x.id }) + 1;
    match *action {
        Add(ref st) => state.push(Todo {
                text: st.clone(),
                completed: false,
                id: max_id
            }),
        Delete(id) => state.retain(|x| x.id != id),
        Edit(id, ref st) => {
            for todo in state.iter_mut() {
                if todo.id == id {
                    todo.text = st.clone();
                }
            }
        }
        Complete(id) => {
            for todo in state.iter_mut() {
                if todo.id == id {
                    todo.completed = !todo.completed;
                }
            }
        }
        CompleteAll => for todo in state.iter_mut() { todo.completed = true },
        ClearCompleted => state.retain(|x| !x.completed),
    }
}

#[test]
fn simple_todo() {
    let state = TodoState::new().reduce(&TodoAction::Add(String::from("Add more tests")));
    assert_eq!(state.todos.len(), 1);
    let state = state.reduce(&TodoAction::Edit(state.todos[0].id, String::from("Even more tests")));
    assert_eq!(state.todos[0].text, "Even more tests");
    let state = state.reduce(&TodoAction::Complete(state.todos[0].id)).reduce(&TodoAction::ClearCompleted);
    assert_eq!(state.todos.len(), 0);
}

#[test]
fn complex_todo() {
    let state = TodoState::new()
        .reduce(&TodoAction::Add(String::from("Add more tests")))
        .reduce(&TodoAction::Add(String::from("Add more tests")))
        .reduce(&TodoAction::Add(String::from("Add more tests")))
        .reduce(&TodoAction::Add(String::from("Add more tests")))
        .reduce(&TodoAction::Add(String::from("Add more tests")))
        .reduce(&TodoAction::Delete(1));
    assert_eq!(state.todos.len(), 4);
    let state = state.reduce(&TodoAction::CompleteAll).reduce(&TodoAction::ClearCompleted);
    assert_eq!(state.todos.len(), 0);
}

