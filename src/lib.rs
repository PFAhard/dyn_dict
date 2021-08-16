use std::collections::HashMap;
use std::any::Any;

pub type Dict<'a> = HashMap<&'a str, Box<dyn Any>>;