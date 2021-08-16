use std::collections::HashMap;
use std::any::Any;
use serde::{Deserialize, Serialize};

pub type Dict<'a> = HashMap<&'a str, Box<dyn AnySer<'a>>>;

trait AnySer<'a>: Any + Deserialize<'a> + Serialize {}