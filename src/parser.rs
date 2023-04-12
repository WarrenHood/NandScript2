use nom::{IResult, sequence::{delimited, tuple}, character::complete::{multispace0, alpha1, digit1, u8}, multi::{fold_many0}, branch::alt, bytes::complete::tag};
use crate::ast::*;

pub fn parse_word(input: &str) -> IResult<&str, String> {
    let (input, (first, rest)) = delimited(multispace0,
        tuple((alpha1, fold_many0(alt((alpha1, digit1)), || String::new(), |acc, item| acc + item))),
        multispace0)
    (input)?;
    Ok((input, format!("{}{}", first, rest)))
}

pub fn parse_var(input: &str) -> IResult<&str, NandScript> {
    let (input, varname) = parse_word(input)?;
    Ok((input, NandScript::Variable(varname)))
}

pub fn parse_literal(input: &str) -> IResult<&str, NandScript> {
    let (input, val) = delimited(multispace0, u8, multispace0)(input)?;
    Ok((input, NandScript::Literal(val)))
}

pub fn parse_expression(input: &str) -> IResult<&str, NandScript> {
    alt((parse_chipcall, parse_var, parse_literal))(input)
}

pub fn parse_chipcall(input: &str) -> IResult<&str, NandScript> {
    let (input, chip_name) = parse_word(input)?;
    let (input, (first_arg, other_args)) = delimited(multispace0, delimited(tag("("), tuple(
        (parse_expression,
        fold_many0(
            tuple((tag(","), parse_expression)),
            || Vec::new(),
            |acc, item| [acc, vec![item.1]].concat()
        )
        )
    ), tag(")")), multispace0)(input)?;
    Ok((input, NandScript::ChipCall(chip_name, Box::new([vec![first_arg], other_args].concat()))))
}

pub fn parse_args(input: &str) -> IResult<&str, Vec<String>> {
    let (input, (first_arg, other_args)) = delimited(multispace0, 
        delimited(tag("("), 
        tuple(
            (parse_word,
            fold_many0(
                tuple((tag(","), parse_word)),
                || Vec::new(),
                |acc, item| [acc, vec![item.1]].concat()
            )
            )
        )
        , tag(")"))
        , multispace0)(input)?;

    Ok((input, [vec![first_arg], other_args].concat()))
}

pub fn parse_chipdef(input: &str) -> IResult<&str, NandScript> {
    let (input, chip_name) = parse_word(input)?;
    let (input, args) = parse_args(input)?;
    let (input, _) = delimited(multispace0, tag("="), multispace0)(input)?;
    let (input, logic) = parse_expression(input)?;
    
    Ok((input, NandScript::ChipDef(chip_name, args, Box::new(logic))))
}

pub fn parse_nandscript(input: &str) -> IResult<&str, NandScript> {
    alt((parse_chipdef, parse_expression))(input)
}