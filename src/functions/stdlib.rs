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
    HashMap::from([
        print.arc_fn("print"),
        bash.arc_fn("bash"),
        bash_status.arc_fn("bash_status"),
        debug.arc_fn("debug"),
    ])
}

fn print(input: &str) -> String {
    println!("{}", input);
    String::new()
}

fn debug(input: &str) -> String {
    eprintln!("{}", input);
    input.to_string()
}

fn bash_status(input: &str) -> String {
    let command = Command::new("bash")
        .args(["-c", &format!("{input}")])
        .stdin(std::process::Stdio::inherit())
        .status();

    match command {
        Ok(status) => status.code().unwrap_or(0).to_string(),
        Err(error) => error.to_string(),
    }
}

fn bash(input: &str) -> String {
    let command = Command::new("bash")
        .args(["-c", &format!("{input}")])
        .stdin(std::process::Stdio::inherit())
        .output();

    match command {
        Ok(output) => std::str::from_utf8(&output.stdout)
            .unwrap()
            .trim()
            .to_string(),
        Err(error) => error.to_string(),
    }
}
