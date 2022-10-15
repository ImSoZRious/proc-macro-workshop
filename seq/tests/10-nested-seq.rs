use seq::seq;

fn independent_loop() -> i32 {
    let mut ans = 0;

    seq!(N in 0..5 {
        seq!(M in 0..5 {
            ans += N * 10 + M;
        });
    });

    ans
}

fn dependent_loop() -> i32 {
    let mut ans = 0;

    seq!(N in 0..5 {
        seq!(M in 0..N {
            ans += N * 10 + M;
        });
    });

    ans
}

fn main() {
    let ans1 = independent_loop();
    let ans2 = dependent_loop();

    assert_eq!(ans1, 550);
    assert_eq!(ans2, 310);
}
