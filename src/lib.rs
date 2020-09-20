use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Object)]
pub struct Constants;

#[methods]
impl Constants {
    pub fn new(_owner: &Object) -> Self { Constants {} }

    #[export]
    pub fn some_constant(&self, _owner: &Object) -> i32 { 42 }
}

fn init(handle: InitHandle) {
    handle.add_class::<Constants>()
}

godot_init!(init);
