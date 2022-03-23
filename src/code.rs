//! Commutative diagram solver.

use crate::Expr;
use crate::Morphism::{self, *};
use crate::sym;

/// Normalize square.
pub fn eval(code: [Morphism; 4]) -> [Morphism; 4] {
    // Helper method for matching against any of the left argument.
    fn or(code: [Morphism; 2], a: Morphism) -> bool {code[0] == a || code[1] == a}
    // Helper method for involving first argument by matching.
    fn read(code: &mut [Morphism; 2], a: Morphism, neg: bool) -> bool {
        if code[0] == a {
            if neg {*code = [code[1], code[0]]};
            true
        } else if code[1] == a {
            if !neg {*code = [code[1], code[0]]};
            true
        } else {
            false
        }
    }

    let ref mut x = [Dir, RevDir];
    if (
        read(x, code[0], false) &&
        or([x[1], Iso], code[1]) &&
        or([x[0], Iso], code[2]) &&
        or([x[1], Iso], code[3])
    ) || (
        Iso == code[0] && (
            (
                read(x, code[1], false) &&
                or([x[1], Iso], code[2]) &&
                or([x[0], Iso], code[3])
            ) || (
                Iso == code[1] && (
                    (
                        read(x, code[2], true) &&
                        or([x[0], Iso], code[3])
                    ) || (
                        Iso == code[2] &&
                        or([Dir, RevDir], code[3])
                    )
                )
            )
        )
    ) {[Iso; 4]} else {code}
}

fn is_reversed(code: Morphism) -> bool {
    match code {
        Dir | Mono | Epi | EpiMono | LeftInv | RightInv | Iso => false,
        RevDir | RevMono | RevEpi | RevEpiMono | RevLeftInv | RevRightInv | RevIso | RevZero => true,
        Unknown | Zero => false,
    }
}

fn reverse(code: Morphism) -> Morphism {
    match code {
        Unknown => Unknown,
        Dir => RevDir,
        RevDir => Dir,
        Mono => RevMono,
        RevMono => Mono,
        Epi => RevEpi,
        RevEpi => Epi,
        EpiMono => RevEpiMono,
        RevEpiMono => EpiMono,
        LeftInv => RevLeftInv,
        RevLeftInv => LeftInv,
        RightInv => RevRightInv,
        RevRightInv => RightInv,
        Iso => RevIso,
        RevIso => Iso,
        Zero => RevZero,
        RevZero => Zero,
    }
}

/// Represents a commutative square.
#[derive(Debug, PartialEq, Eq)]
pub struct Square {
    /// Bound expressions.
    pub bind: Vec<Expr>,
    /// Labels of edges.
    ///
    /// Stores an object `[<obj>, 0, 0]`
    /// or an edge `[0, <from>, <to>]`.
    pub labels: [[u8; 3]; 4],
    /// Morphism codes of the edges.
    pub code: [Morphism; 4],
}

impl Square {
    /// Creates a new square.
    pub fn new(expr: &Expr) -> Option<Self> {
        fn find(bind: &mut Vec<Expr>, a: &Expr) -> u8 {
            for (i, e) in bind.iter().enumerate() {
                if e == a {return (i + 1) as u8}
            }
            bind.push(a.clone());
            bind.len() as u8
        }
        fn new(bind: &mut Vec<Expr>, a: &Expr) -> [u8; 3] {
            match a {
                Mor(_, _, b) => {
                    let n = find(bind, &b.0);
                    let m = find(bind, &b.1);
                    [0, n, m]
                }
                _ => [find(bind, a), 0, 0],
            }
        }
        fn code(a: &Expr, edge: usize, labels: &mut [[u8; 3]; 4]) -> Morphism {
            let f = (
                edge == 1 &&
                labels[0][1] != 0 &&
                labels[edge][2] == labels[0][1]
            ) || (
                edge == 2 &&
                labels[0][2] != 0 &&
                labels[edge][2] == labels[0][2]
            ) || (
                edge == 3 &&
                labels[1][2] != 0 &&
                labels[edge][2] == labels[1][2]
            ) || (
                edge == 3 &&
                labels[2][2] != 0 &&
                labels[edge][1] == labels[2][2]
            );

            if let Mor(mor, _, b) = a {
                match mor {
                    Unknown => Unknown,
                    Dir | Mono | Epi if b.0 == b.1 => Iso,
                    x if edge == 0 => *x,
                    x => if f {
                        // Swap end-points to match morphism.
                        let [a, b, c] = labels[edge];
                        labels[edge] = [a, c, b];
                        reverse(*x)
                    } else {*x},
                }
            } else {Unknown}
        }

        use crate::Expr::*;
        use crate::Morphism::*;

        let mut bind: Vec<Expr> = vec![];
        if let Mor(Iso, 2, a) = expr {
            if let Path(ltb) = &a.0 {
                match &ltb.1 {
                    Mor(_, _, tb) => {
                        let mut labels = [
                            new(&mut bind, &ltb.0),
                            new(&mut bind, &tb.0),
                            new(&mut bind, &tb.1),
                            new(&mut bind, &a.1)
                        ];
                        let code = [
                            code(&ltb.0, 0, &mut labels),
                            code(&tb.0, 1, &mut labels),
                            code(&tb.1, 2, &mut labels),
                            code(&a.1, 3, &mut labels),
                        ];
                        Some(Square {labels, bind, code})
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Evaluates square.
    pub fn eval(&self) -> [Morphism; 4] {
        fn update_code(code: Morphism, av: &Arc<String>) -> Morphism {
            match (code, &***av) {
                (Dir, "mor") => code,
                (Dir, "mono") => Mono,
                (Dir, "zero") => Zero,
                (Dir, "left_inv") => code,
                (Dir, "right_inv") => RightInv,
                (RevDir, "mor") => code,
                (RevDir, "mono") => RevMono,
                (RevDir, "iso") => RevIso,
                (RevDir, "zero") => RevZero,
                (RevDir, "right_inv") => code,
                (RevDir, "left_inv") => RevLeftInv,
                (Mono, "mono" | "mor") => code,
                (Mono, "epi") => EpiMono,
                (Mono, "zero") => Zero,
                (RevMono, "mor") => code,
                (RevMono, "mono") => code,
                (RevMono, "epi") => RevEpiMono,
                (RevMono, "zero") => RevZero,
                (Epi, "epi" | "mor") => code,
                (Epi, "left_inv" | "right_inv") => code,
                (Epi, "mono") => EpiMono,
                (Epi, "iso") => Iso,
                (RevEpi, "epi" | "mor") => code,
                (RevEpi, "mono") => RevEpiMono,
                (RevEpi, "iso") => RevIso,
                (EpiMono, "epi" | "mono") => code,
                (EpiMono, "left_inv" | "right_inv") => code,
                (EpiMono, "iso") => Iso,
                (RevEpiMono, "mor" | "epi" | "mono") => code,
                (RevEpiMono, "left_inv" | "right_inv") => code,
                (RevEpiMono, "iso") => RevIso,
                (LeftInv, "mor" | "mono" | "left_inv") => code,
                (LeftInv, "right_inv") => Iso,
                (RevLeftInv, "mor" | "mono" | "left_inv") => code,
                (RevLeftInv, "right_inv") => RevIso,
                (RightInv, "mor" | "epi" | "right_inv") => code,
                (RightInv, "mono") => code,
                (RightInv, "left_inv" | "iso") => Iso,
                (RevRightInv, "mor" | "epi" | "right_inv") => code,
                (RevRightInv, "mono") => code,
                (RevRightInv, "left_inv") => RevIso,
                (Iso, "iso" | "left_inv" | "right_inv" | "mono" | "epi" | "mor") => code,
                (Iso, "zero") => Zero,
                (RevIso, "iso" | "left_inv" | "right_inv" | "mono" | "epi" | "mor") => code,
                (RevIso, "zero") => RevZero,
                (Zero, "zero" | "iso" | "left_inv" | "right_inv" | "mono" | "epi" | "mor") => code,
                (RevZero, "zero" | "iso" | "left_inv" | "right_inv" | "mono" | "epi" | "mor") => code,
                _ => unimplemented!("{:?} {}", code, av),
            }
        }

        use std::sync::Arc;
        use avalog::{infer, parse_str, rel, ava, solve_with_accelerator, Accelerator, ParseData};

        let ref mut acc = Accelerator::new();
        let ref parent = match std::env::current_dir() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Could not get working directory");
                return self.code;
            }
        };
        let mut start: ParseData<sym::Sym> = parse_str(include_str!("../assets/cat.txt"), parent).unwrap();

        let zero: sym::Sym = Arc::new("zero".to_string()).into();
        let iso: sym::Sym = Arc::new("iso".to_string()).into();
        let left_inv: sym::Sym = Arc::new("left_inv".to_string()).into();
        let right_inv: sym::Sym = Arc::new("right_inv".to_string()).into();
        let mono: sym::Sym = Arc::new("mono".to_string()).into();
        let epi: sym::Sym = Arc::new("epi".to_string()).into();
        let mor: sym::Sym = Arc::new("mor".to_string()).into();

        for i in 0..4 {
            match self.labels[i] {
                [0, a, b] => {
                    let a: avalog::Expr<sym::Sym> = avalog::Expr::Sym(self.bind[(a - 1) as usize].clone().into());
                    let b: avalog::Expr<sym::Sym> = avalog::Expr::Sym(self.bind[(b - 1) as usize].clone().into());
                    let (a, b) = if is_reversed(self.code[i]) {(b, a)} else {(a, b)};
                    match self.code[i] {
                        Unknown => {}
                        Dir | RevDir => start.push(rel(a, ava(mor.clone(), b))),
                        Iso | RevIso => start.push(rel(a, ava(iso.clone(), b))),
                        Mono | RevMono => start.push(rel(a, ava(mono.clone(), b))),
                        Zero | RevZero => start.push(rel(a, ava(zero.clone(), b))),
                        Epi | RevEpi => start.push(rel(a, ava(epi.clone(), b))),
                        EpiMono | RevEpiMono => {
                            start.push(rel(a.clone(), ava(epi.clone(), b.clone())));
                            start.push(rel(a, ava(mono.clone(), b)));
                        }
                        LeftInv | RevLeftInv => start.push(rel(a, ava(left_inv.clone(), b))),
                        RightInv | RevRightInv => start.push(rel(a, ava(right_inv.clone(), b))),
                    }
                }
                _ => {}
            }
        }

        let res = solve_with_accelerator(
            &start,
            &[avalog::Expr::Ambiguity(false)],
            None,
            &[],
            &[],
            infer,
            acc,
        );

        let find = |name: &Expr| -> Option<u8> {
            for (i, n) in self.bind.iter().enumerate() {
                if n == name {return Some((i + 1) as u8)};
            }
            None
        };

        let mut new_code = self.code;
        for x in &res.0 {
            use avalog::Expr::*;

            match x {
                Rel(a, b) => {
                    match (&**a, &**b) {
                        (Sym(a), Ava(av, b)) => {
                            if let sym::Sym::Expr(a) = a {
                                if let Some(a_ind) = find(a) {
                                    if let (Sym(av), Sym(b)) = (&**av, &**b) {
                                        if let (sym::Sym::Ava(av), sym::Sym::Expr(b)) = (av, b) {
                                            if let Some(b_ind) = find(b) {
                                                for i in 0..4 {
                                                    if self.labels[i][1] == a_ind &&
                                                       self.labels[i][2] == b_ind {
                                                           new_code[i] = update_code(new_code[i], av);
                                                    }
                                                    if self.labels[i][2] == b_ind &&
                                                       self.labels[i][1] == a_ind {
                                                           new_code[i] = update_code(new_code[i], av);
                                                       }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        new_code
    }

    /// Update square.
    pub fn update(&self, e: &mut Expr) {
        fn fix(mor: &mut Morphism, code: Morphism, ab: &mut Arc<(Expr, Expr)>) {
            if is_reversed(code) != is_reversed(*mor) {
                *ab = Arc::new((ab.1.clone(), ab.0.clone()));
            }
            *mor = code;
        }

        use crate::Expr::*;
        use crate::Morphism::*;
        use std::sync::Arc;

        let new_code = self.eval();
        if let Mor(Iso, 2, a) = e {
            let a = Arc::make_mut(a);
            if let Mor(mor, _, ab) = &mut a.1 {
                fix(mor, new_code[3], ab);
            }
            if let Path(ltb) = &mut a.0 {
                let ltb = Arc::make_mut(ltb);
                if let Mor(mor, _, _) = &mut ltb.0 {
                    *mor = new_code[0];
                }
                if let Mor(_, _, tb) = &mut ltb.1 {
                    let tb = Arc::make_mut(tb);
                    if let Mor(mor, _, ab) = &mut tb.0 {
                        fix(mor, new_code[1], ab);
                    }
                    if let Mor(mor, _, ab) = &mut tb.1 {
                        fix(mor, new_code[2], ab);
                    }
                }
            }
        }
    }
}
