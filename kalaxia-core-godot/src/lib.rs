use gdnative::prelude::*;
use kalaxia_core::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Constants;

#[methods]
impl Constants {
    pub fn new(_owner: &Node) -> Self { Constants }

    #[export]
    pub fn some_constant(&self, _owner: &Node) -> i32 { SOME_CONSTANT }
}

fn init(handle: InitHandle) {
    handle.add_class::<Constants>()
}

godot_init!(init);
