/// The stack module, which contains the stack and different functions to control the stack.
pub mod stack;

/// The frames module, which contains the frame struct. Frames are used when the VM jumps to a label.
pub mod frames;

/// The store module, which provides a nice wrapper around a HashMap for maintaining variables.
pub mod store;

pub mod label;

pub mod parameter;
