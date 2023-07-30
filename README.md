In Rust procedual macros you define them using this pattern `$name:type`, where name is the name of the parameter
and type is one of 'literal', 'expr', 'ident' or 'ty' if I remember correctly.

So if you have a macro where you need to repeat some arguments any amount of times then you can simply do
something like this.

```rust 
macro_rules! amazing_macro {
    ($( //* out your parameters here */ ),*) => {}
}
```

```rust
macro_rules! some_weird_macro {
    ($($name:ident -> $handle:ident),*) => {
        $(
            println!("{:?} -> {:?}", $name, $handle);
        )*
    };
}
```
