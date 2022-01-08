/// Lookup table generated with:
/// ```rust
/// let mut buffer = [[(0, 0, 0);8];8];
/// for x in 0..8{
///     for y in 0..8{
///         let len = ((x*x + y*y) as f32).sqrt();
///         buffer[x][y] = (
///             (x as f32/len).round() as i8,
///             (y as f32/len).round() as i8,
///             (len * 13.5).round() as i8
///         );
///     }
/// }
/// println!("{:?}", buffer)
/// ```
pub const VCL_TABLE: [[(i8, i8, i8); 8]; 8] = [
    [
        (0, 0, 0),
        (0, 1, 13),
        (0, 1, 26),
        (0, 1, 38),
        (0, 1, 51),
        (0, 1, 64),
        (0, 1, 77),
        (0, 1, 90),
    ],
    [
        (1, 0, 13),
        (1, 1, 18),
        (0, 1, 29),
        (0, 1, 40),
        (0, 1, 53),
        (0, 1, 65),
        (0, 1, 78),
        (0, 1, 91),
    ],
    [
        (1, 0, 26),
        (1, 0, 29),
        (1, 1, 36),
        (1, 1, 46),
        (0, 1, 57),
        (0, 1, 69),
        (0, 1, 81),
        (0, 1, 93),
    ],
    [
        (1, 0, 38),
        (1, 0, 40),
        (1, 1, 46),
        (1, 1, 54),
        (1, 1, 64),
        (1, 1, 75),
        (0, 1, 86),
        (0, 1, 97),
    ],
    [
        (1, 0, 51),
        (1, 0, 53),
        (1, 0, 57),
        (1, 1, 64),
        (1, 1, 72),
        (1, 1, 82),
        (1, 1, 92),
        (0, 1, 103),
    ],
    [
        (1, 0, 64),
        (1, 0, 65),
        (1, 0, 69),
        (1, 1, 75),
        (1, 1, 82),
        (1, 1, 91),
        (1, 1, 100),
        (1, 1, 110),
    ],
    [
        (1, 0, 77),
        (1, 0, 78),
        (1, 0, 81),
        (1, 0, 86),
        (1, 1, 92),
        (1, 1, 100),
        (1, 1, 109),
        (1, 1, 118),
    ],
    [
        (1, 0, 90),
        (1, 0, 91),
        (1, 0, 93),
        (1, 0, 97),
        (1, 0, 103),
        (1, 1, 110),
        (1, 1, 118),
        (1, 1, 127),
    ],
];

use crate::Pos;
pub fn vector_comp(pos: &Pos, mv: &Pos) -> (i8, i8, i8) {
    let dx = pos.x - mv.x;
    let dy = pos.y - mv.y;

    let mut comps = unsafe {
        // The magic of vector component lookup tables (which is something i just made up)
        // lets us avoid wasting cpu time on calculating the square root of the same numbers over and over again.
        *VCL_TABLE
            .get_unchecked(dx.unsigned_abs() as usize)
            .get_unchecked(dy.unsigned_abs() as usize)
    };
    comps.0 *= dx.signum(); // Sign copying so we don't need a big symetrical lookup table.
    comps.1 *= dy.signum();
    comps
}

// This replaces the old, slow, function:
// ```rust
//    let vector_comp = |pos: &Pos, mv: &Pos| -> (f32, f32, f32) {
//        let mut dx = (pos.x - mv.x) as f32;
//        let mut dy = (pos.y - mv.y) as f32;
//        let len = ((dx.powi(2) + dy.powi(2)) as f32).sqrt();
//        dx = if len == 0. { 0. } else { dx / len };
//        dy = if len == 0. { 0. } else { dy / len };
//        (dx.round(), dy.round(), len)
//    };
// ```
