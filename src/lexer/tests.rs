use nom::{
    character::complete::{none_of, one_of},
    CompareResult, Parser,
};

use super::*;

fn tuple_assert<A, B>(generated: (A, B), expected: (A, B))
where
    A: nom::Compare<A>,
    B: nom::Compare<B>,
{
    let output = generated.1.compare(expected.1);
    let residue = expected.0.compare(generated.0);

    assert_eq!(residue, CompareResult::Ok);
    assert_eq!(output, CompareResult::Ok);
}

#[test]
fn parse_string() {
    let special_character = ":+ ;@\\";
    let acceptable = none_of(special_character);
    let escapable = one_of(special_character);

    let input = "lost\\+found";
    let fallible_output =
        parse_str_abstract::<error::VerboseError<&str>, _, _, _, _>(acceptable, escapable)
            .parse(input);
    tuple_assert(fallible_output.unwrap(), ("", "lost\\+found"))
}

#[test]
#[ignore]
fn parse_dependency() {
    let sample = "zoxide: cargo + fzf;";
    let _package = parse_dependency_tree::<error::VerboseError<&str>>(sample);
}
