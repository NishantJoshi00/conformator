use color_eyre::{eyre::bail, Result};
use std::{collections::HashMap, sync::Arc};

use crate::types;

pub fn function_registry_composer<'a, 'b>(
    functions: types::Functions<'a>,
) -> HashMap<&'a str, Arc<dyn Fn(&'b str) -> String + 'a>> {
    HashMap::new()
}

pub fn function_interpreter<'a: 'b, 'b>(
    fun: types::Function<'a>,
    function_dir: &'a HashMap<&'a str, Arc<dyn Fn(&str) -> String>>,
) -> Result<Arc<dyn Fn(&'b str) -> String + 'b>> {
    match fun.args {
        types::Arg::String(value) => format_string_consumer(value),
        types::Arg::Expr(expr) => expression_consumer(expr, &function_dir),
    }
}

pub fn expression_consumer<'a: 'b, 'b>(
    expression: Box<types::Expr<'a>>,
    function_dir: &'a HashMap<&'a str, Arc<dyn Fn(&str) -> String>>,
) -> Result<Arc<dyn Fn(&'b str) -> String + 'b>> {
    match expression.argument {
        types::Arg::String(value) => format_string_consumer(value),
        types::Arg::Expr(inner_expr) => {
            let parent_function = match function_dir.get(expression.name) {
                Some(inner) => inner.clone(),
                None => bail!("there is no function in the scope: {}", expression.name),
            };

            let inner_expression = expression_consumer(inner_expr, function_dir)?;

            Ok(Arc::new(move |x| {
                let inner = (inner_expression)(x);
                let x = (parent_function)(&inner);
                x
            }))
        }
    }
}

pub fn format_string_consumer<'a, 'b>(_data: &'a str) -> Result<Arc<dyn Fn(&'b str) -> String>> {
    todo!()
}
