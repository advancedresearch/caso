//! Parse syntax.

use crate::*;

use piston_meta::{Convert, Range};

fn parse_expr(
    node: &str,
    mut convert: Convert,
    ignored: &mut Vec<Range>
) -> Result<(Range, Expr), ()> {
    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut expr: Option<Expr> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, _)) = convert.meta_bool("0") {
            convert.update(range);
            expr = Some(_0);
        } else if let Ok((range, val)) = convert.meta_string("obj") {
            convert.update(range);
            expr = Some(Obj(val));
        } else if let Ok((range, val)) = parse_mor("mor", convert, ignored) {
            convert.update(range);
            expr = Some(val);
        } else if let Ok((range, val)) = parse_expr("path", convert, ignored) {
            convert.update(range);
            if let Some(obj) = expr {
                expr = Some(Path(Arc::new((obj, val))));
            } else {
                return Err(());
            }
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let expr = expr.ok_or(())?;
    Ok((convert.subtract(start), expr))
}

fn parse_mor(
    node: &str,
    mut convert: Convert,
    ignored: &mut Vec<Range>
) -> Result<(Range, Expr), ()> {
    let start = convert;
    let start_range = convert.start_node(node)?;
    convert.update(start_range);

    let mut mor: Option<Morphism> = None;
    let mut n = 0;
    let mut left: Option<Expr> = None;
    let mut right: Option<Expr> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = parse_expr("left", convert, ignored) {
            convert.update(range);
            left = Some(val);
        } else if let Ok((range, val)) = parse_expr("right", convert, ignored) {
            convert.update(range);
            right = Some(val);
        } else if let Ok((range, _)) = convert.meta_bool("iso") {
            convert.update(range);
            mor = Some(Iso);
        } else if let Ok((range, _)) = convert.meta_bool("dir") {
            convert.update(range);
            mor = Some(Dir);
        } else if let Ok((range, _)) = convert.meta_bool("rev_dir") {
            convert.update(range);
            mor = Some(RevDir);
        } else if let Ok((range, _)) = convert.meta_bool("zero") {
            convert.update(range);
            mor = Some(Zero);
        } else if let Ok((range, _)) = convert.meta_bool("mono") {
            convert.update(range);
            mor = Some(Mono);
        } else if let Ok((range, _)) = convert.meta_bool("rev_mono") {
            convert.update(range);
            mor = Some(RevMono);
        } else if let Ok((range, _)) = convert.meta_bool("epi") {
            convert.update(range);
            mor = Some(Epi);
        } else if let Ok((range, _)) = convert.meta_bool("rev_epi") {
            convert.update(range);
            mor = Some(RevEpi);
        } else if let Ok((range, _)) = convert.meta_bool("epi_mono") {
            convert.update(range);
            mor = Some(EpiMono);
        } else if let Ok((range, _)) = convert.meta_bool("rev_epi_mono") {
            convert.update(range);
            mor = Some(RevEpiMono);
        } else if let Ok((range, _)) = convert.meta_bool("left_inv") {
            convert.update(range);
            mor = Some(LeftInv);
        } else if let Ok((range, _)) = convert.meta_bool("rev_left_inv") {
            convert.update(range);
            mor = Some(RevLeftInv);
        } else if let Ok((range, _)) = convert.meta_bool("right_inv") {
            convert.update(range);
            mor = Some(RightInv);
        } else if let Ok((range, _)) = convert.meta_bool("rev_right_inv") {
            convert.update(range);
            mor = Some(RevRightInv);
        } else if let Ok((range, _)) = convert.meta_bool("2-dir") {
            convert.update(range);
            mor = Some(Dir);
        } else if let Ok((range, _)) = convert.meta_bool("2-iso") {
            convert.update(range);
            mor = Some(Iso);
        } else if let Ok((range, _)) = convert.meta_bool("+2") {
            convert.update(range);
            n += 2;
        } else if let Ok((range, _)) = convert.meta_bool("+1") {
            convert.update(range);
            n += 1;
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let mor = mor.ok_or(())?;
    let left = left.ok_or(())?;
    let right = right.ok_or(())?;
    let n = if mor == Zero {1} else {n};
    let expr = Mor(mor, n, Arc::new((left, right)));
    Ok((convert.subtract(start), expr))
}

/// Parses an expression string.
pub fn parse_str(data: &str) -> Result<Expr, String> {
    use piston_meta::{parse_errstr, syntax_errstr};

    let syntax_src = include_str!("../assets/syntax.txt");
    let syntax = syntax_errstr(syntax_src)?;

    let mut meta_data = vec![];
    parse_errstr(&syntax, &data, &mut meta_data)?;

    // piston_meta::json::print(&meta_data);

    let convert = Convert::new(&meta_data);
    let mut ignored = vec![];
    match parse_expr("expr", convert, &mut ignored) {
        Err(()) => Err("Could not convert meta data".into()),
        Ok((_, expr)) => Ok(expr),
    }
}
