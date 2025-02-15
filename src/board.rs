use std::{cmp, fmt};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, Serialize, Deserialize)]
pub struct Board(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for j in 0..3 {
            writeln!(f, "+--+--+--+")?;
            for i in 0..3 {
                write!(f, "|")?;
                if self.get(i, j) == 0 {
                    write!(f, "  ")?;
                } else {
                    write!(f, "{} ", self.get(i, j))?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+--+--+--+")
    }
}

impl Board {
    pub fn empty() -> Self {
        Board(0)
    }
    pub fn create(vals: &[u8; 9]) -> Self {
        let mut out = Board::empty();
        for i in 0..3 {
            for j in 0..3 {
                out.set(i, j, vals[i + j * 3]);
            }
        }
        out
    }
    pub fn canonical_orientation(self) -> Self {
        let res = self;
        let res = cmp::min(res, self.rotate_clockwise::<1>());
        let res = cmp::min(res, self.rotate_clockwise::<2>());
        let res = cmp::min(res, self.rotate_clockwise::<3>());
        let flip = self.reflect_horizontal();
        let res = cmp::min(res, flip);
        let res = cmp::min(res, flip.rotate_clockwise::<1>());
        let res = cmp::min(res, flip.rotate_clockwise::<2>());
        let res = cmp::min(res, flip.rotate_clockwise::<3>());
        res
    }
    fn get(&self, x: usize, y: usize) -> u8 {
        let res = self.0 >> (4 * (x + y * 3));
        let res = res & 15;
        res as u8
    }
    pub(crate) fn is_win(&self) -> bool {
        let mut sum = 0usize;
        for i in 0..3 {
            for j in 0..3 {
                sum += self.get(i, j) as usize;
            }
        }
        return sum == 45;
    }
    pub(crate) fn is_loss(&self) -> bool {
        for m in [Move::Up, Move::Down, Move::Left, Move::Right] {
            if self.do_move(m).is_some() {
                return false;
            }
        }
        return !self.is_win();
    }
    fn set(&mut self, x: usize, y: usize, val: u8) {
        let shift = 4 * (x + y * 3);
        let mask = !(15 << shift);
        self.0 &= mask;
        self.0 |= (val as u64) << shift;
    }
    fn rotate_clockwise<const N: u8>(self) -> Self {
        let mut out = Board::empty();
        for i in 0..3 {
            for j in 0..3 {
                let (mut ir, mut jr) = (i, j);
                for _ in 0..N {
                    let (i2, j2) = (jr, 2 - ir);

                    (ir, jr) = (i2, j2);
                }
                out.set(i, j, self.get(ir, jr));
            }
        }
        out
    }
    fn reflect_horizontal(self) -> Self {
        let mut out = Board::empty();
        for i in 0..3 {
            for j in 0..3 {
                out.set(i, j, self.get(2 - i, j));
            }
        }
        out
    }
    pub fn do_move(&self, dir: Move) -> Option<Board> {
        let mut out = match dir {
            Move::Right => self.rotate_clockwise::<2>(),
            Move::Left => self.rotate_clockwise::<0>(),
            Move::Down => self.rotate_clockwise::<1>(),
            Move::Up => self.rotate_clockwise::<3>(),
        };
        for i in 0..3 {
            let row = Self::shift_line(&[out.get(0, i), out.get(1, i), out.get(2, i)]);
            out.set(0, i, row[0]);
            out.set(1, i, row[1]);
            out.set(2, i, row[2]);
        }
        let ret = match dir {
            Move::Right => out.rotate_clockwise::<2>(),
            Move::Left => out.rotate_clockwise::<0>(),
            Move::Down => out.rotate_clockwise::<3>(),
            Move::Up => out.rotate_clockwise::<1>(),
        };
        if &ret == self {
            None
        } else {
            Some(ret)
        }
    }

    pub fn possible_moves(&self) -> impl Iterator<Item = (Board, Move)> {
        let s = *self;
        [Move::Up, Move::Down, Move::Left, Move::Right]
            .iter()
            .filter_map(move |m| s.do_move(*m).map(|b| (b, *m)))
    }

    pub fn possible_new_tiles(&self) -> impl Iterator<Item = Board> {
        let b = *self;
        (0..9)
            .map(move |i| (i / 3, i % 3, b))
            .filter(|(x, y, b)| b.get(*x, *y) == 0)
            .map(|(x, y, b)| {
                let mut b: Board = b;
                b.set(x, y, 1);
                b
            })
    }

    fn shift_line(vals: &[u8; 3]) -> [u8; 3] {
        let mut out = [0u8; 3];
        let mut next_idx = 0;
        // Perform shifts
        for i in 0..vals.len() {
            if vals[i] == 0 {
                continue;
            }
            out[next_idx] = vals[i];
            next_idx += 1;
        }
        // Combine tiles
        if out[0] == out[1] && out[0] != 0 {
            out[0] += 1;
            out[1] = out[2];
            out[2] = 0;
        } else if out[1] == out[2] && out[1] != 0 {
            out[1] += 1;
            out[2] = 0;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn do_move() {
        let x = Board::create(&[1u8; 9]);
        assert_eq!(
            x.do_move(Move::Up),
            Some(Board::create(&[
                2, 2, 2, //
                1, 1, 1, //
                0, 0, 0, //
            ]))
        );
        assert_eq!(
            x.do_move(Move::Down),
            Some(Board::create(&[
                0, 0, 0, //
                1, 1, 1, //
                2, 2, 2, //
            ]))
        );
        assert_eq!(
            x.do_move(Move::Left),
            Some(Board::create(&[
                2, 1, 0, //
                2, 1, 0, //
                2, 1, 0, //
            ]))
        );
        assert_eq!(
            x.do_move(Move::Right),
            Some(Board::create(&[
                0, 1, 2, //
                0, 1, 2, //
                0, 1, 2, //
            ]))
        );
    }
    #[test]
    fn rotate_clockwise() {
        let x = Board::create(&[
            1, 2, 3, //
            4, 5, 6, //
            7, 8, 9, //
        ]);
        let y = Board::create(&[
            7, 4, 1, //
            8, 5, 2, //
            9, 6, 3, //
        ]);
        assert_eq!(x.rotate_clockwise::<1>(), y);
    }
    #[test]
    fn reflect_horizontal() {
        let x = Board::create(&[
            1, 2, 3, //
            4, 5, 6, //
            7, 8, 9, //
        ]);
        let y = Board::create(&[
            3, 2, 1, //
            6, 5, 4, //
            9, 8, 7, //
        ]);
        assert_eq!(x.reflect_horizontal(), y);
    }
    #[test]
    fn shift_line() {
        assert_eq!(Board::shift_line(&[0, 0, 0]), [0, 0, 0]);
        assert_eq!(Board::shift_line(&[1, 0, 1]), [2, 0, 0]);
        assert_eq!(Board::shift_line(&[0, 1, 1]), [2, 0, 0]);
        assert_eq!(Board::shift_line(&[1, 1, 1]), [2, 1, 0]);
        assert_eq!(Board::shift_line(&[2, 1, 1]), [2, 2, 0]);
    }
}
