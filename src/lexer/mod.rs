#[cfg(test)]
mod tests;

use nom::{
    branch,
    bytes::complete as bcomplete,
    character::complete,
    combinator, error, multi,
    sequence::{self, preceded},
    IResult, Parser,
};

use crate::types;

const SPECIAL_CHARACTERS: &str = ":+ ;@$\\'\"";

// ----------------- primitive parsers ---------------------- //

fn parse_str_abstract<'a, E, F1, F2, O1, O2>(
    acceptable: F1,
    escapable: F2,
) -> impl Parser<&'a str, &'a str, E>
where
    E: error::ParseError<&'a str>,
    F1: Parser<&'a str, O1, E>,
    F2: Parser<&'a str, O2, E>,
{
    bcomplete::escaped(acceptable, '\\', escapable)
}

pub(crate) fn parse_str<'a, E: error::ParseError<&'a str>>(
    special_character: &'a str,
) -> impl Parser<&'a str, &'a str, E> {
    parse_str_abstract(
        complete::none_of(special_character),
        complete::one_of(special_character),
    )
}

fn parse_space_safe<'a, F: 'a, O, E>(parser: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: error::ParseError<&'a str>,
{
    sequence::delimited(complete::multispace0, parser, complete::multispace0)
}

// ----------------- domain specific parsers ---------------------- //

fn parse_identifier<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str> + 'a,
{
    let delimited =
        |x, y| sequence::delimited(complete::char(x), parse_str::<E>(y), complete::char(x));
    error::context(
        "identifier",
        parse_space_safe(branch::alt((
            delimited('"', "\""),
            delimited('\'', "'"),
            parse_str(SPECIAL_CHARACTERS),
        ))),
    )
    .parse(input)
}

fn parse_dependency_tree<'a, E>(input: &'a str) -> IResult<&'a str, types::Package<'a>, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str> + 'a,
{
    error::context(
        "dependency_tree",
        parse_space_safe(sequence::terminated(
            sequence::separated_pair(
                error::context(
                    "package",
                    sequence::pair(
                        parse_identifier,
                        combinator::opt(sequence::preceded(complete::char('@'), parse_identifier)),
                    ),
                ),
                complete::char(':'),
                error::context(
                    "dependencies",
                    multi::separated_list1(complete::char('+'), parse_identifier),
                ),
            ),
            complete::char(';'),
        ))
        .map(|(x, y)| (x, y.into_iter().collect()))
        .map(types::Package::from_tuple),
    )
    .parse(input)
}

fn parse_function_definition<'a, E>(input: &'a str) -> IResult<&'a str, types::Function<'a>, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str> + 'a,
{
    error::context(
        "function definition",
        parse_space_safe(sequence::delimited(
            bcomplete::tag("--"),
            parse_space_safe(sequence::separated_pair(
                parse_identifier,
                bcomplete::tag("does"),
                parse_function_expression,
            )),
            complete::char(';'),
        )),
    )
    .map(types::Function::from_tuple)
    .parse(input)
}

fn parse_function_expression<'a, E>(input: &'a str) -> IResult<&'a str, types::Arg<'a>, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str> + 'a,
{
    error::context(
        "function body",
        parse_space_safe(branch::alt((
            preceded(
                complete::char('$'),
                sequence::pair(
                    parse_identifier,
                    branch::alt((
                        parse_function_expression,
                        parse_identifier.map(types::Arg::String),
                    )),
                ),
            )
            .map(types::Expr::boxed_from_tuple)
            .map(types::Arg::Expr),
            parse_identifier.map(types::Arg::String),
        ))),
    )
    .parse(input)
}

pub fn parser_prime<'a, E>(input: &'a str) -> IResult<&'a str, Vec<types::Concept<'a>>, E>
where
    E: error::ParseError<&'a str> + error::ContextError<&'a str> + 'a,
{
    error::context(
        "concept builder",
        parse_space_safe(multi::many0(branch::alt((
            parse_function_definition.map(Into::into),
            parse_dependency_tree.map(Into::into),
        )))),
    )
    .parse(input)
}
