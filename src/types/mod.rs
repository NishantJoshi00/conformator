use std::{collections::HashSet, fmt::Display};
pub mod fn_types;

pub type Packages<'a> = HashSet<Package<'a>>;
pub type Functions<'a> = Vec<Function<'a>>;

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Concept<'a> {
    Package(Package<'a>),
    Function(Function<'a>),
}

#[derive(Debug, Default)]
pub struct Concepts<'a> {
    pub packages: Packages<'a>,
    pub functions: Functions<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Package<'a> {
    pub name: &'a str,
    pub dependency: Vec<&'a str>,
    pub processor: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub args: Arg<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expr<'a> {
    pub name: &'a str,
    pub argument: Arg<'a>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Arg<'a> {
    String(&'a str),
    Expr(Box<Expr<'a>>), // todo: find a indirection to allow non-allocated expression
}

impl<'a> std::hash::Hash for Expr<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> std::hash::Hash for Function<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> std::hash::Hash for Package<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> Package<'a> {
    pub(crate) fn from_tuple(
        ((name, processor), dependency): ((&'a str, Option<&'a str>), Vec<&'a str>),
    ) -> Self {
        Self {
            name,
            dependency,
            processor,
        }
    }

    pub(crate) fn from_name(name: &'a str) -> Self {
        Self {
            name,
            dependency: Default::default(),
            processor: Default::default(),
        }
    }
}

impl<'a> From<Package<'a>> for Concept<'a> {
    fn from(value: Package<'a>) -> Self {
        Self::Package(value)
    }
}

impl<'a> From<Function<'a>> for Concept<'a> {
    fn from(value: Function<'a>) -> Self {
        Self::Function(value)
    }
}

impl<'a> From<Vec<Concept<'a>>> for Concepts<'a> {
    fn from(value: Vec<Concept<'a>>) -> Self {
        value
            .into_iter()
            .fold(Concepts::default(), |mut initial, new_concept| {
                match new_concept {
                    Concept::Package(package) => {
                        initial.packages.insert(package);
                    }
                    Concept::Function(function) => {
                        initial.functions.push(function);
                    }
                };
                initial
            })
    }
}

impl<'a> Expr<'a> {
    pub fn boxed_from_tuple((name, args): (&'a str, Arg<'a>)) -> Box<Self> {
        Box::new(Self {
            name,
            argument: args,
        })
    }
}

impl<'a> Function<'a> {
    pub fn from_tuple((name, args): (&'a str, Arg<'a>)) -> Self {
        Self { name, args }
    }
}

impl<'a> Display for Package<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}
