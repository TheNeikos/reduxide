use std::default::Default;
#[macro_use] extern crate reduxide;
extern crate cursive;

use std::thread::{self, Thread};
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{ListView, Checkbox, LinearLayout, EditView, Dialog};
use std::sync::mpsc::channel;

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
        CompleteAll => for todo in state.iter_mut() { todo.completed = !todo.completed },
        ClearCompleted => state.retain(|x| !x.completed),
    }
}

fn main() {
    let mut state = TodoState::new();
    let mut siv = Cursive::new();
    siv.set_fps(30);

    let sink = siv.cb_sink().clone();
    let sub = state.subscribe();
    let (tx, rx) = channel::<TodoAction>();


    let tx_c = tx.clone();
    let tx_d = tx.clone();
    let tx_e = tx.clone();

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(EditView::new()
                   .on_submit(move |ref mut s, new_str|{
                       tx_c.send(TodoAction::Add(String::from(new_str))).unwrap();
                       let mut entry = s.find_id::<EditView>("todo_entry").unwrap();
                       entry.set_content("");
                   }).with_id("todo_entry"))
            .child(ListView::new().with_id("todo_list")))
        .title("Todo List")
        .button("Complete All", move|ref mut s| {
            tx_d.send(TodoAction::CompleteAll);
        })
        .button("Clear Completed", move|ref mut s| {
            tx_e.send(TodoAction::ClearCompleted);
        }));

    let tx_c = tx.clone();
    thread::spawn(move || {
        let sub = sub;
        let tx = tx_c;
        while let Ok(state) = sub.recv() {
            let todos = state.todos.clone();
            let tx_c = tx.clone();
            sink.send(Box::new(move |ref mut s| {
                let list = s.find_id::<ListView>("todo_list").unwrap();
                list.clear();
                for todo in &todos {
                    let tx_c = tx_c.clone();
                    let id : u64 = todo.id;
                    let mut checkbox = Checkbox::new();
                    checkbox.set_checked(todo.completed);
                    checkbox.set_on_change(move |ref mut s, new| {
                        tx_c.send(TodoAction::Complete(id)).unwrap();
                    });
                    list.add_child(&todo.text, checkbox);
                }
            }));
        }
    });

    thread::spawn(move || {
        let rx = rx;

        let mut state = state;
        while let Ok(action) = rx.recv() {
            state = state.reduce(&action);
        }
    });

    siv.run();
}
