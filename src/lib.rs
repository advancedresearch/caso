#![deny(missing_docs)]

//! # Caso
//!
//! Category Theory Solver for Commutative Diagrams.
//!
//!
//! ```text
//! === Caso 0.1 ===
//! Type `help` for more information.
//! > (A <-> B)[(A <-> C) -> (B <-> D)] <=> (C -> D)
//! (A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)
//! ```
//!
//! To run Case from your Terminal, type:
//!
//! `cargo install --example caso caso`
//!
//! Then, to run:
//!
//! `caso`
//!
//! | Morphism | Notation |
//! | --- | --- |
//! | Directional | `->` |
//! | Reverse Directional | `<-` |
//! | Epi | `->>` |
//! | Reverse Epi | `<<-` |
//! | Mono | `!->` |
//! | Reverse Mono | `<-!` |
//! | Left Inverse | `<!->` |
//! | Reverse Left Inverse | `<-!>` |
//! | Right Inverse | `<->>` |
//! | Reverse Right Inverse | `<<->` |
//! | Epi-Mono | `!->>` |
//! | Reverse Epi-Mono | `<<-!` |
//! | Iso | `<->` |
//! | Zero | `<>` |

use std::sync::Arc;
use std::fmt;

use Expr::*;
use Morphism::*;

pub mod parsing;
pub mod code;
pub mod sym;

/// Represents a morphism.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
#[repr(u8)]
pub enum Morphism {
    /// Unknown morphism.
    Unknown,
    /// Directional.
    Dir,
    /// Reverse directional.
    RevDir,
    /// Mono.
    Mono,
    /// Reverse mono.
    RevMono,
    /// Epi.
    Epi,
    /// Reverse epi.
    RevEpi,
    /// Epi-mono.
    EpiMono,
    /// Reverse epi-mono.
    RevEpiMono,
    /// Left inverse.
    LeftInv,
    /// Reverse left inverse.
    RevLeftInv,
    /// Right inverse.
    RightInv,
    /// Reverse right inverse.
    RevRightInv,
    /// Iso.
    Iso,
    /// Reverse iso.
    RevIso,
    /// Zero.
    Zero,
    /// Reverse zero.
    RevZero,
}

/// Stores Caso expression.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum Expr {
    /// A zero object.
    _0,
    /// An object.
    Obj(Arc<String>),
    /// A morphism.
    Mor(Morphism, usize, Arc<(Expr, Expr)>),
    /// A path.
    Path(Arc<(Expr, Expr)>),
}

impl fmt::Display for Expr {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fn needs_parens(e: &Expr) -> bool {
            match e {
                Mor(_, _, _) => true,
                _ => false,
            }
        }

        match self {
            _0 => write!(w, "0")?,
            Obj(x) => write!(w, "{}", x)?,
            Mor(mor, n, a) => {
                if needs_parens(&a.0) {write!(w, "({}) ", a.0)?}
                else {write!(w, "{} ", a.0)?}
                write!(w, "{}", match mor {
                    Zero | RevZero | Iso | RevIso | RevDir | RevMono | RevLeftInv | RightInv => "<",
                    Mono | EpiMono => "!",
                    LeftInv => "<!",
                    RevEpi | RevEpiMono | RevRightInv => "<<",
                    Unknown | Dir | Epi => "",
                })?;
                if mor != &Zero {
                    for _ in 0..*n / 2 {write!(w, "=")?}
                    for _ in 0..*n % 2 {write!(w, "-")?}
                }
                write!(w, "{}", match mor {
                    Dir | Zero | RevZero | Iso | RevIso | Mono | LeftInv | RevRightInv => ">",
                    Epi | RightInv | EpiMono => ">>",
                    RevMono | RevEpiMono => "!",
                    RevLeftInv => "!>",
                    Unknown | RevDir | RevEpi => ""
                })?;
                if needs_parens(&a.1) {write!(w, " ({})", a.1)?}
                else {write!(w, " {}", a.1)?}
            }
            Path(a) => {
                let parens = needs_parens(&a.0);
                if parens {write!(w, "({})", a.0)?}
                else {write!(w, "{}", a.0)?};
                write!(w, "[{}]", a.1)?;
            }
        }
        Ok(())
    }
}

impl Expr {
    /// Returns left edge, if any.
    pub fn left(&self) -> Option<Expr> {
        match self {
            Path(a) => Some(a.0.clone()),
            _ => None,
        }
    }

    /// Returns top edge, if any.
    pub fn top(&self) -> Option<Expr> {
        match self {
            Path(a) => match &a.1 {
                Mor(_, _, b) => Some(b.0.clone()),
                _ => None,
            }
            _ => None,
        }
    }

    /// Returns bottom edge, if any.
    pub fn bottom(&self) -> Option<Expr> {
        match self {
            Path(a) => match &a.1 {
                Mor(_, _, b) => Some(b.1.clone()),
                _ => None,
            }
            _ => None,
        }
    }
}

impl<'a> TryFrom<&'a str> for Expr {
    type Error = String;
    fn try_from(val: &'a str) -> Result<Expr, String> {
        match parsing::parse_str(val) {
            Ok(x) => Ok(x),
            Err(err) => Err(format!("ERROR:\n{}", err)),
        }
    }
}

/// Higher iso e.g. `A <=> B`.
pub fn iso_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(Iso, n, Arc::new((a, b)))
}

/// Iso e.g. `A <-> B`.
pub fn iso(a: Expr, b: Expr) -> Expr {iso_n(1, a, b)}

/// Higher directional e.g. `A => B`.
pub fn dir_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(Dir, n, Arc::new((a, b)))
}

/// Directional e.g. `A -> B`.
pub fn dir(a: Expr, b: Expr) -> Expr {dir_n(1, a, b)}

/// Higher zero.
pub fn zero_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(Zero, n, Arc::new((a, b)))
}

/// Zero e.g. `A <> B`.
pub fn zero(a: Expr, b: Expr) -> Expr {zero_n(1, a, b)}

/// Higher mono e.g. `A !=> B`.
pub fn mono_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(Mono, n, Arc::new((a, b)))
}

/// Mono e.g. `A !-> B`.
pub fn mono(a: Expr, b: Expr) -> Expr {mono_n(1, a, b)}

/// Higher reverse mono e.g. `A <=! B`.
pub fn rev_mono_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(RevMono, n, Arc::new((a, b)))
}

/// Reverse mono e.g. `A <-! B`.
pub fn rev_mono(a: Expr, b: Expr) -> Expr {rev_mono_n(1, a, b)}

/// Higher epi e.g. `A =>> B`.
pub fn epi_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(Epi, n, Arc::new((a, b)))
}

/// Epi e.g. `A ->> B`.
pub fn epi(a: Expr, b: Expr) -> Expr {epi_n(1, a, b)}

/// Reverse epi-mono e.g. `A <<=! B`.
pub fn rev_epi_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(RevEpi, n, Arc::new((a.into(), b.into())))
}

/// Higher epi-mono e.g. `A !=>> B`.
pub fn epi_mono_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(EpiMono, n, Arc::new((a, b)))
}

/// Reverse epi-mono e.g. `A <<-! B`.
pub fn rev_epi(a: Expr, b: Expr) -> Expr {rev_epi_n(1, a, b)}

/// Epi-mono e.g. `A !->> B`.
pub fn epi_mono(a: Expr, b: Expr) -> Expr {epi_mono_n(1, a, b)}

/// Higher left inverse e.g. `A <!=> B`.
pub fn left_inv_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(LeftInv, n, Arc::new((a, b)))
}

/// Left inverse e.g. `A <!-> B`.
pub fn left_inv(a: Expr, b: Expr) -> Expr {left_inv_n(1, a, b)}

/// Higher reverse left inverse e.g. `A <=!> B`.
pub fn rev_left_inv_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(RevLeftInv, n, Arc::new((a.into(), b.into())))
}

/// Reverse left inverse e.g. `A <-!> B`.
pub fn rev_left_inv(a: Expr, b: Expr) -> Expr {rev_left_inv_n(1, a, b)}

/// Higher right inverse e.g. `A <=>> B`.
pub fn right_inv_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(RightInv, n, Arc::new((a, b)))
}

/// Right inverse e.g. `A <->> B`.
pub fn right_inv(a: Expr, b: Expr) -> Expr {right_inv_n(1, a, b)}

/// Higher reverse right inverse e.g. `A <<=> B`.
pub fn rev_right_inv_n(n: usize, a: Expr, b: Expr) -> Expr {
    Mor(RevRightInv, n, Arc::new((a, b)))
}

/// Reverse right inverse e.g. `A <<-> B`.
pub fn rev_right_inv(a: Expr, b: Expr) -> Expr {rev_right_inv_n(1, a, b)}

/// A path e.g. `X[Y]`.
pub fn path(a: Expr, b: Expr) -> Expr {
    Path(Arc::new((a, b)))
}

/// Solve a string.
///
/// Returns the solution as a string.
/// Produces an error message if the solver failed.
pub fn solve_str(a: &str) -> Result<String, String> {
    let mut a: Expr = a.try_into()?;
    let sq = code::Square::new(&a).ok_or("Could not convert into square".to_string())?;
    sq.update(&mut a);
    Ok(format!("{}", a))
}

/// Converts string into expression (panics when format is invalid).
pub fn conv(a: &str) -> Expr {a.try_into().unwrap()}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: Expr = conv("X");
        assert_eq!(a, Obj(Arc::new("X".into())));

        let b = iso(conv("X"), conv("Y"));
        assert_eq!(b, Mor(Iso, 1, Arc::new((conv("X"), conv("Y")))));

        let c = dir(conv("X"), conv("Y"));
        assert_eq!(c, Mor(Dir, 1, Arc::new((conv("X"), conv("Y")))));

        let b2: Expr = conv("X <-> Y");
        assert_eq!(b2, b);

        let b3: Expr = conv("X -> Y");
        assert_eq!(b3, dir(conv("X"), conv("Y")));

        let b4: Expr = conv("X <> Y");
        assert_eq!(b4, zero(conv("X"), conv("Y")));

        let b5: Expr = conv("X => Y");
        assert_eq!(b5, dir_n(2, conv("X"), conv("Y")));

        let b6: Expr = conv("X <=> Y");
        assert_eq!(b6, iso_n(2, conv("X"), conv("Y")));

        let b7: Expr = conv("X ->> Y");
        assert_eq!(b7, epi(conv("X"), conv("Y")));

        let b8: Expr = conv("X !->> Y");
        assert_eq!(b8, epi_mono(conv("X"), conv("Y")));

        let b9: Expr = conv("X <!-> Y");
        assert_eq!(b9, left_inv(conv("X"), conv("Y")));

        let b10: Expr = conv("X <->> Y");
        assert_eq!(b10, right_inv(conv("X"), conv("Y")));

        let b11: Expr = conv("X <<- Y");
        assert_eq!(b11, rev_epi(conv("X"), conv("Y")));

        let b12: Expr = conv("X <-! Y");
        assert_eq!(b12, rev_mono(conv("X"), conv("Y")));

        let b13: Expr = conv("X <-!> Y");
        assert_eq!(b13, rev_left_inv(conv("X"), conv("Y")));

        let b14: Expr = conv("X <<-> Y");
        assert_eq!(b14, rev_right_inv(conv("X"), conv("Y")));

        let b15: Expr = conv("X <==> Y");
        assert_eq!(b15, iso_n(4, conv("X"), conv("Y")));

        let b16: Expr = conv("X <-=> Y");
        assert_eq!(b16, iso_n(3, conv("X"), conv("Y")));

        let b17: Expr = conv("X <--> Y");
        assert_eq!(b17, iso_n(2, conv("X"), conv("Y")));

        let d: Expr = conv("X[Y]");
        assert_eq!(d, path(conv("X"), conv("Y")));

        let e: Expr = conv("f[g] <=> h");
        assert_eq!(e, iso_n(2, path(conv("f"), conv("g")), conv("h")));

        let z: Expr = conv("0");
        assert_eq!(z, _0);
    }

    #[test]
    fn normalise() {
        use code::Square;

        let a: Expr = conv("(a -> b)[(c -> a) -> (b -> d)]");
        assert_eq!(a.left().unwrap(), conv("a -> b"));
        assert_eq!(a.top().unwrap(), conv("c -> a"));
        assert_eq!(a.bottom().unwrap(), conv("b -> d"));

        let cd: Expr = conv("c <-> d");
        let b = iso_n(2, a, cd);
        let sq = Square::new(&b).unwrap();
        assert_eq!(sq, Square {
            bind: vec![conv("a"), conv("b"), conv("c"), conv("d")],
            labels: [[0, 1, 2], [0, 3, 1], [0, 2, 4], [0, 3, 4]],
            code: [Dir, RevDir, Dir, Iso],
        });
        assert_eq!(code::eval(sq.code), [Iso; 4]);

        let a: Expr = conv("(a -> b)[(b -> c) -> (d -> b)] <=> (c -> d)");
        assert_eq!(Square::new(&a).unwrap().code, [Dir, Dir, RevDir, Dir]);
    }

    #[test]
    fn test_eval() {
        let a: Expr = conv("f[(X !-> 1) -> (0 !-> Y)] <=> (0 <-> 1)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Unknown, Mono, Mono, RevIso]);

        let a: Expr = conv("(X <> Y)[(X !-> 1) -> (0 !-> Y)] <=> (0 <-> 1)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Zero, Mono, RevMono, RevIso]);
        assert_eq!(sq.eval(), [Zero, Zero, RevZero, RevZero]);

        let a: Expr = conv("(X -> Y)[(X !-> 1) -> (0 !-> Y)] <=> (0 <-> 1)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, Mono, RevMono, RevIso]);
        assert_eq!(sq.eval(), [Zero, Zero, RevZero, RevZero]);

        let a: Expr = conv("(X -> Y)[(X !-> 1) -> (0 -> Y)] <=> (0 <-> 1)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, Mono, RevDir, RevIso]);
        assert_eq!(sq.eval(), [Zero, Zero, RevZero, RevZero]);

        let a: Expr = conv("(A ->> B)[(C ->> A) -> (B ->> D)] <=> (D ->> C)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Epi, RevEpi, Epi, RevEpi]);
        assert_eq!(sq.eval(), [EpiMono, RevEpiMono, EpiMono, RevEpiMono]);

        let a: Expr = conv("(A -> B)[(C -> A) -> (B -> D)] <=> (D -> C)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, RevDir, Dir, RevDir]);
        assert_eq!(sq.eval(), [Dir, RevDir, Dir, RevDir]);

        let a: Expr = conv("(A -> B)[(C -> A) -> (B -> D)] <=> (D <-> C)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, RevDir, Dir, RevIso]);
        assert_eq!(sq.eval(), [Dir, RevDir, Dir, RevIso]);

        let a: Expr = conv("(A <-> B)[(C <-> A) -> (B <-> D)] <=> (C -> D)");
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Iso, RevIso, Iso, Dir]);
        assert_eq!(sq.eval(), [Iso, RevIso, Iso, Iso]);
    }

    #[test]
    fn format() {
        fn check(a: &str) {
            let b: Expr = conv(a);
            assert_eq!(&format!("{}", b), a);
        }

        check("0");
        check("X");
        check("X -> Y");
        check("X <- Y");
        check("X <-> Y");
        check("X <=> Y");
        check("X <=-> Y");
        check("X ->> Y");
        check("X <<- Y");
        check("X !-> Y");
        check("X <-! Y");
        check("X <!-> Y");
        check("X <-!> Y");
        check("X <->> Y");
        check("X <<-> Y");
        check("X <<- Y");
        check("X !->> Y");
        check("X <<-! Y");
        check("X <> Y");
        check("X[Y]");
        check("(A -> A)[A -> B]");
        check("(A -> A)[A -> B] <=> (B -> B)");
        check("(B -> B) <=> (A -> A)[A -> B]");
        check("(A -> B)[(A -> C) -> (B -> D)] <=> (C -> D)");
    }

    #[test]
    fn rewrite() {
        // `(C -> D) => (C <-> D)`.
        let ref a = solve_str("(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C -> D)").unwrap();
        assert_eq!(a, "(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)");

        // `(D -> C) => (C <-> D)`.
        let ref a = solve_str("(A <-> B)[(A <-> C) -> (B <-> D)] <=> (D -> C)").unwrap();
        assert_eq!(a, "(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)");

        // `(A -> B) => (A <-> B)`.
        let ref a = solve_str("(A -> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)").unwrap();
        assert_eq!(a, "(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)");

        // `(A -> C) => (A <-> C)`.
        let ref a = solve_str("(A <-> B)[(A -> C) -> (B <-> D)] <=> (C <-> D)").unwrap();
        assert_eq!(a, "(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)");

        // `(C -> A) => (A <-> C)`.
        let ref a = solve_str("(A <-> B)[(C -> A) -> (B <-> D)] <=> (C <-> D)").unwrap();
        assert_eq!(a, "(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)");

        // `(B -> D) => (B <-> D)`
        let ref a = solve_str("(A <-> B)[(A <-> C) -> (B -> D)] <=> (C <-> D)").unwrap();
        assert_eq!(a, "(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)");

        // `(D -> B) => (B <-> D)`
        let ref a = solve_str("(A <-> B)[(A <-> C) -> (D -> B)] <=> (C <-> D)").unwrap();
        assert_eq!(a, "(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)");
    }
}
