use crate::*;

/// Used to represent symbols in avalog.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum Sym {
    /// Avatar.
    Ava(Arc<String>),
    /// An epxression.
    Expr(Expr),
}

impl fmt::Display for Sym {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Sym::Ava(v) => write!(w, "{}", v),
            Sym::Expr(v) => write!(w, "{}", v),
        }
    }
}

impl From<Arc<String>> for Sym {
    fn from(val: Arc<String>) -> Sym {
        match &**val {
            "0" => Sym::Expr(Expr::_0),
            _ => Sym::Ava(val),
        }
    }
}

impl From<Expr> for Sym {
    fn from(val: Expr) -> Sym {Sym::Expr(val)}
}

impl avalog::IsVar for Sym {}

impl Into<avalog::Expr<Sym>> for Sym {
    fn into(self) -> avalog::Expr<Sym> {
        avalog::Expr::Sym(self)
    }
}
