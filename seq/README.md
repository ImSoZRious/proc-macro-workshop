# Note
Since the workshop doesn't state how to categorize when to expand all or only section. For example.
```rs
seq!(N in 0..16 {
#[derive(Copy, Clone, PartialEq, Debug)]
enum Interrupt {
    #(
        Irq~N,
    )*
}
```
should be expand to
```rs
#[derive(Copy, Clone, PartialEq, Debug)]
enum Interrupt {
  Irq1,
  ...,
  Irq15
}
```
not
```rs
#[derive(Copy, Clone, PartialEq, Debug)]
enum Interrupt {
  Irq1,
  ...,
  Irq15
}
enum Interrupt {
  Irq1,
  ...,
  Irq15
}
...
enum Interrupt {
  Irq1,
  ...,
  Irq15
}
```
I am going to make general assumption that if there exists at least one `#()*` token pattern, it's going to expand only the section. Else it should expand all as if there was `#()*` token surround the block.
For example.
```rs
seq!(N in 0..4 {
    compile_error!(concat!("error number ", stringify!(N)));
});
```
should be same as
```rs
seq!(N in 0..4 {
    #(
        compile_error!(concat!("error number ", stringify!(N)));
    )*
});
```