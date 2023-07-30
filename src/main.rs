macro_rules! some_weird_macro {
    ($($name:ident -> $handle:ident),*) => {
        $(
            println!("{:?} -> {:?}", $name, $handle);
        )*
    };
}

fn main() {
    let spell = "Fire ball";
    let spell_handle = "fire_ball";

    some_weird_macro!(
        spell -> spell_handle,
        spell -> spell_handle
    );
}
