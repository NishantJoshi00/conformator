use std::{sync::Arc, collections::HashMap};


pub(crate) type ArcFn<'a> = Arc<dyn Fn(&str) -> String + 'a>;

pub(crate) type FnMap<'a> = HashMap<&'a str, ArcFn<'a>>;
