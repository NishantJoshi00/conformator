use crate::types::fn_types;
use std::{collections::HashMap, process::Command, sync::Arc};

trait ArcFnTy<'a>
where
    Self: for<'b> Fn(&'b str) -> String + 'a,
{
    fn arc_fn<'b>(&'a self, name: &'b str) -> (&'b str, fn_types::ArcFn<'a>);
}

impl<'a, T: for<'b> Fn(&'b str) -> String + 'a> ArcFnTy<'a> for T {
    fn arc_fn<'b>(&'a self, name: &'b str) -> (&'b str, fn_types::ArcFn<'a>) {
        (name, Arc::new(self))
    }
}

pub(super) fn standard_functions<'a>() -> fn_types::FnMap<'a> {
    HashMap::from([print.arc_fn("print"), bash.arc_fn("bash")])
}

fn print(input: &str) -> String {
    println!("{}", input);
    String::new()
}

fn bash(input: &str) -> String {
    let command = Command::new("bash")
        .args(["-c", &format!("\"{input}\"")])
        .stdin(std::process::Stdio::inherit())
        .status();
    match command {
        Ok(status) => status.to_string(),
        Err(error) => error.to_string(),
    }
}
