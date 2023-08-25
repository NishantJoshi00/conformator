use color_eyre::{eyre::bail, Result};
use nom::Parser;
use std::sync::Arc;

use crate::types::{self, fn_types};

mod stdlib;
use stdlib::standard_functions;

pub fn function_registry_composer(functions: types::Functions) -> Result<fn_types::FnMap> {
    functions
        .into_iter()
        .fold(Ok(standard_functions()), move |acc, fun| -> Result<_> {
            match acc {
                Ok(mut dir) => {
                    let fun_name = fun.name;
                    let fun_ty = function_interpreter(fun, &dir)?;
                    dir.insert(fun_name, fun_ty);
                    Ok(dir)
                }
                error @ Err(_) => error,
            }
        })
}

pub fn function_interpreter<'a>(
    fun: types::Function<'a>,
    function_dir: &fn_types::FnMap<'a>,
) -> Result<fn_types::ArcFn<'a>> {
    match fun.args {
        types::Arg::String(value) => match function_dir.get(value) {
            Some(inner) => Ok(inner.clone()),
            None => format_string_consumer(value),
        },
        types::Arg::Expr(expr) => expression_consumer(*expr, function_dir),
    }
}

pub fn expression_consumer<'a>(
    expression: types::Expr<'a>,
    function_dir: &fn_types::FnMap<'a>,
) -> Result<fn_types::ArcFn<'a>> {
    let parent_function = match function_dir.get(expression.name) {
        Some(inner) => inner.clone(),
        None => bail!("there is no function in the scope: {}", expression.name),
    };
    let inner_expression = match expression.argument {
        types::Arg::String(value) => match function_dir.get(value) {
            Some(inner) => inner.clone(),
            None => format_string_consumer(value)?,
        },
        types::Arg::Expr(inner_expr) => expression_consumer(*inner_expr, function_dir)?,
    };

    Ok(Arc::new(move |x| {
        let inner = (inner_expression)(x);
        (parent_function)(&inner)
    }))
}

pub fn format_string_consumer(input: &str) -> Result<fn_types::ArcFn> {
    let string_pair = nom::sequence::separated_pair(
        crate::lexer::parse_str::<()>("{"),
        nom::bytes::complete::tag("{}"),
        crate::lexer::parse_str::<()>(""),
    )
    .parse(input);

    match string_pair {
        Ok(("", (part_1, part_2))) => Ok(Arc::new(move |input| {
            format!("{}{}{}", part_1, input, part_2)
        })),
        Ok((a, (_, _))) => {
            bail!("The lexer couldn't parse the string: {}", a)
        }
        Err(_) => Ok(Arc::new(move |_| input.to_string())),
    }
}
