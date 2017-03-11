#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
//!
//! Reduxide is a heavily [redux][redux] inspired library.
//! It's basic premise is state management through a specific data flow.
//! To achieve this a 'data pipeline' is constructed from side-effect free functions
//! that are chained together. A side-effect free function is one that does not
//! modify any outside state and always gives the same output for the same input.
//!
//! A high-level example:
//!
//! ```
//! use std::default::Default;
//! #[macro_use] extern crate reduxide;
//! state! {
//!     State,
//!     [
//!         health: Health > HealthActions > health_reducer
//!     ]
//! }
//!
//! #[derive(Clone)]
//! struct Health {
//!     sick: bool,
//!     hp: u8,
//! }
//!
//! impl Default for Health {
//!     fn default() -> Health {
//!         Health {
//!             sick: false,
//!             hp: 100,
//!         }
//!     }
//! }
//!
//! enum HealthActions {
//!     Add(u8),
//!     Sub(u8),
//!     Set(u8),
//! }
//!
//! fn health_reducer(state: &mut Health, action: &HealthActions) {
//!     match *action {
//!         HealthActions::Add(amt) => state.hp += amt,
//!         HealthActions::Sub(amt) => state.hp -= amt,
//!         HealthActions::Set(amt) => state.hp = amt,
//!     }
//! }
//!
//! fn main() {
//!     let state = State::new();
//!     let new_state = state.reduce(&HealthActions::Set(200))
//!         .reduce(&HealthActions::Sub(10))
//!         .reduce(&HealthActions::Add(10));
//!     assert_eq!(new_state.health.hp, 200);
//! }
//! ```
//!
//!


#[macro_export]
macro_rules! state {
    ( $name:ident , [$($field:ident : $struct:ty > $action:ident > $reducer:ident),*]) => {
        #[derive(Clone, Default)]
        struct $name {
            subs: Vec<::std::sync::mpsc::Sender<$name>>,
            $(
                $field : $struct,
            )*
        }

        impl $name {
            fn new() -> Self {
                Self::default()
            }

            fn subscribe(&mut self) -> ::std::sync::mpsc::Receiver<Self> {
                let (tx, rx) = ::std::sync::mpsc::channel();
                self.subs.push(tx);
                return rx;
            }

            fn reduce<T: ::std::any::Any>(&self, action: &T) -> Self {
                let action = action as &::std::any::Any;
                let mut new_self = self.clone();
                $(
                    if let Some(action) = action.downcast_ref::<$action>() {
                        $reducer(&mut new_self.$field, action);
                    }
                )*
                {
                    let mut subs = new_self.subs;
                    new_self.subs = Vec::new();
                    subs.retain(|sub| sub.send(new_self.clone()).is_ok());
                    new_self.subs = subs;
                }
                return new_self;
            }
        }
    }
}

