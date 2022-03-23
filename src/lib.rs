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
    Unknown,
    Dir,
    RevDir,
    Mono,
    RevMono,
    Epi,
    RevEpi,
    EpiMono,
    RevEpiMono,
    LeftInv,
    RevLeftInv,
    RightInv,
    RevRightInv,
    Iso,
    RevIso,
    Zero,
    RevZero,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum Expr {
    _0,
    Obj(Arc<String>),
    Mor(Morphism, usize, Arc<(Expr, Expr)>),
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
    pub fn left(&self) -> Option<Expr> {
        match self {
            Path(a) => Some(a.0.clone()),
            _ => None,
        }
    }

    pub fn top(&self) -> Option<Expr> {
        match self {
            Path(a) => match &a.1 {
                Mor(_, _, b) => Some(b.0.clone()),
                _ => None,
            }
            _ => None,
        }
    }

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

impl<'a> From<&'a str> for Expr {
    fn from(val: &'a str) -> Expr {
        match parsing::parse_str(val) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("ERROR:\n{}", err);
                panic!()
            }
        }
    }
}

pub fn iso_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(Iso, n, Arc::new((a.into(), b.into())))
}

pub fn iso<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {iso_n(1, a, b)}


pub fn dir_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(Dir, n, Arc::new((a.into(), b.into())))
}

pub fn dir<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {dir_n(1, a, b)}

pub fn zero_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(Zero, n, Arc::new((a.into(), b.into())))
}

pub fn zero<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {zero_n(1, a, b)}

pub fn mono_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(Mono, n, Arc::new((a.into(), b.into())))
}

pub fn mono<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {mono_n(1, a, b)}

pub fn rev_mono_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(RevMono, n, Arc::new((a.into(), b.into())))
}

pub fn rev_mono<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {rev_mono_n(1, a, b)}

pub fn epi_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(Epi, n, Arc::new((a.into(), b.into())))
}

pub fn epi<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {epi_n(1, a, b)}

pub fn rev_epi_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(RevEpi, n, Arc::new((a.into(), b.into())))
}

pub fn epi_mono_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(EpiMono, n, Arc::new((a.into(), b.into())))
}

pub fn rev_epi<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {rev_epi_n(1, a, b)}

pub fn epi_mono<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {epi_mono_n(1, a, b)}

pub fn left_inv_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(LeftInv, n, Arc::new((a.into(), b.into())))
}

pub fn left_inv<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {left_inv_n(1, a, b)}

pub fn rev_left_inv_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(RevLeftInv, n, Arc::new((a.into(), b.into())))
}

pub fn rev_left_inv<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {rev_left_inv_n(1, a, b)}

pub fn right_inv_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(RightInv, n, Arc::new((a.into(), b.into())))
}

pub fn right_inv<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {right_inv_n(1, a, b)}

pub fn rev_right_inv_n<A: Into<Expr>, B: Into<Expr>>(n: usize, a: A, b: B) -> Expr {
    Mor(RevRightInv, n, Arc::new((a.into(), b.into())))
}

pub fn rev_right_inv<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {rev_right_inv_n(1, a, b)}

pub fn path<A: Into<Expr>, B: Into<Expr>>(a: A, b: B) -> Expr {
    Path(Arc::new((a.into(), b.into())))
}

pub fn solve_str(a: &str) -> Option<String> {
    let mut a: Expr = a.into();
    let sq = code::Square::new(&a)?;
    sq.update(&mut a);
    Some(format!("{}", a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: Expr = "X".into();
        assert_eq!(a, Obj(Arc::new("X".into())));

        let b = iso("X", "Y");
        assert_eq!(b, Mor(Iso, 1, Arc::new(("X".into(), "Y".into()))));

        let c = dir("X", "Y");
        assert_eq!(c, Mor(Dir, 1, Arc::new(("X".into(), "Y".into()))));

        let b2: Expr = "X <-> Y".into();
        assert_eq!(b2, b);

        let b3: Expr = "X -> Y".into();
        assert_eq!(b3, dir("X", "Y"));

        let b4: Expr = "X <> Y".into();
        assert_eq!(b4, zero("X", "Y"));

        let b5: Expr = "X => Y".into();
        assert_eq!(b5, dir_n(2, "X", "Y"));

        let b6: Expr = "X <=> Y".into();
        assert_eq!(b6, iso_n(2, "X", "Y"));

        let b7: Expr = "X ->> Y".into();
        assert_eq!(b7, epi("X", "Y"));

        let b8: Expr = "X !->> Y".into();
        assert_eq!(b8, epi_mono("X", "Y"));

        let b9: Expr = "X <!-> Y".into();
        assert_eq!(b9, left_inv("X", "Y"));

        let b10: Expr = "X <->> Y".into();
        assert_eq!(b10, right_inv("X", "Y"));

        let b11: Expr = "X <<- Y".into();
        assert_eq!(b11, rev_epi("X", "Y"));

        let b12: Expr = "X <-! Y".into();
        assert_eq!(b12, rev_mono("X", "Y"));

        let b13: Expr = "X <-!> Y".into();
        assert_eq!(b13, rev_left_inv("X", "Y"));

        let b14: Expr = "X <<-> Y".into();
        assert_eq!(b14, rev_right_inv("X", "Y"));

        let b15: Expr = "X <==> Y".into();
        assert_eq!(b15, iso_n(4, "X", "Y"));

        let b16: Expr = "X <-=> Y".into();
        assert_eq!(b16, iso_n(3, "X", "Y"));

        let b17: Expr = "X <--> Y".into();
        assert_eq!(b17, iso_n(2, "X", "Y"));

        let d: Expr = "X[Y]".into();
        assert_eq!(d, path("X", "Y"));

        let e: Expr = "f[g] <=> h".into();
        assert_eq!(e, iso_n(2, path("f", "g"), "h"));

        let z: Expr = "0".into();
        assert_eq!(z, _0);
    }

    #[test]
    fn normalise() {
        use code::Square;

        let a: Expr = "(a -> b)[(c -> a) -> (b -> d)]".into();
        assert_eq!(a.left().unwrap(), "a -> b".into());
        assert_eq!(a.top().unwrap(), "c -> a".into());
        assert_eq!(a.bottom().unwrap(), "b -> d".into());

        let b = iso_n(2, a, "c <-> d");
        let sq = Square::new(&b).unwrap();
        assert_eq!(sq, Square {
            bind: vec!["a".into(), "b".into(), "c".into(), "d".into()],
            labels: [[0, 1, 2], [0, 3, 1], [0, 2, 4], [0, 3, 4]],
            code: [Dir, RevDir, Dir, Iso],
        });
        assert_eq!(code::eval(sq.code), [Iso; 4]);

        let a: Expr = "(a -> b)[(b -> c) -> (d -> b)] <=> (c -> d)".into();
        assert_eq!(Square::new(&a).unwrap().code, [Dir, Dir, RevDir, Dir]);
    }

    #[test]
    fn test_eval() {
        let a: Expr = "f[(X !-> 1) -> (0 !-> Y)] <=> (0 <-> 1)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Unknown, Mono, Mono, RevIso]);

        let a: Expr = "(X <> Y)[(X !-> 1) -> (0 !-> Y)] <=> (0 <-> 1)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Zero, Mono, RevMono, RevIso]);
        assert_eq!(sq.eval(), [Zero, Zero, RevZero, RevZero]);

        let a: Expr = "(X -> Y)[(X !-> 1) -> (0 !-> Y)] <=> (0 <-> 1)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, Mono, RevMono, RevIso]);
        assert_eq!(sq.eval(), [Zero, Zero, RevZero, RevZero]);

        let a: Expr = "(X -> Y)[(X !-> 1) -> (0 -> Y)] <=> (0 <-> 1)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, Mono, RevDir, RevIso]);
        assert_eq!(sq.eval(), [Zero, Zero, RevZero, RevZero]);

        let a: Expr = "(A ->> B)[(C ->> A) -> (B ->> D)] <=> (D ->> C)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Epi, RevEpi, Epi, RevEpi]);
        assert_eq!(sq.eval(), [EpiMono, RevEpiMono, EpiMono, RevEpiMono]);

        let a: Expr = "(A -> B)[(C -> A) -> (B -> D)] <=> (D -> C)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, RevDir, Dir, RevDir]);
        assert_eq!(sq.eval(), [Dir, RevDir, Dir, RevDir]);

        let a: Expr = "(A -> B)[(C -> A) -> (B -> D)] <=> (D <-> C)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Dir, RevDir, Dir, RevIso]);
        assert_eq!(sq.eval(), [Dir, RevDir, Dir, RevIso]);

        let a: Expr = "(A <-> B)[(C <-> A) -> (B <-> D)] <=> (C -> D)".into();
        let sq = code::Square::new(&a).unwrap();
        assert_eq!(sq.code, [Iso, RevIso, Iso, Dir]);
        assert_eq!(sq.eval(), [Iso, RevIso, Iso, Iso]);
    }

    #[test]
    fn format() {
        fn check(a: &str) {
            let b: Expr = a.into();
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
