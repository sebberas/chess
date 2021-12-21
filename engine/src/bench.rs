/// # Initial
/// <pre>
/// test deepblue::bench::initial_best_move ... bench:   8,348,355 ns/iter (+/- 165,188)
/// test deepblue::bench::move_piece        ... bench:          46 ns/iter (+/- 9)
/// </pre>
/// ![](https://www.dropbox.com/s/kivli6oeazrlcct/flamegraph.svg?raw=1)
///
/// # Sqaure Root Lookup
/// <pre>
/// test deepblue::bench::initial_best_move ... bench:   6,064,273 ns/iter (+/- 2,627,124)
/// test deepblue::bench::move_piece        ... bench:          31 ns/iter (+/- 3)
/// </pre>
/// ![](https://www.dropbox.com/s/754bvzcbmxom4hs/flamegraph2.svg?raw=1)
///
/// # Vector Composition Lookup
/// <pre>
/// test bench::bench::initial_best_move ... bench:   4,336,291 ns/iter (+/- 409,809)
/// test bench::bench::move_piece        ... bench:          29 ns/iter (+/- 1)
/// </pre>

#[cfg(not(target_family = "wasm"))]
pub mod bench {
    extern crate test;
    use crate::GameState;
    use test::Bencher;

    #[bench]
    pub fn initial_best_move(b: &mut Bencher) {
        b.iter(|| {
            let mut game = GameState::default();
            let action = game.best_move(crate::White, 2);
            test::black_box(game.move_piece(action));
        })
    }

    #[bench]
    fn move_piece(b: &mut Bencher) {
        let mut game = GameState::default();
        let action = game.best_move(crate::White, 2);
        b.iter(|| {
            test::black_box(game.move_piece(action));
        })
    }
}
