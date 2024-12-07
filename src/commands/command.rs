// src/commands/command.rs
//
// A Command trait for implementing undoable actions.

pub trait Command {
    fn apply(&self);
    fn undo(&self);
}
