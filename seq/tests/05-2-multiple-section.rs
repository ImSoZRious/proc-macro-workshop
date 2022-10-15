use seq::seq;

seq!(N in 0..16 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Interrupt {
        #(
            Irq~N,
        )*
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Pin {
        #(
            Pin~N,
        )*
    }
});

fn main() {
    let interrupt = Interrupt::Irq8;
    let pin = Pin::Pin5;

    assert_eq!(interrupt as u8, 8);
    assert_eq!(interrupt, Interrupt::Irq8);

    assert_eq!(pin as u8, 5);
    assert_eq!(pin, Pin::Pin5);
}
