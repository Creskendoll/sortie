use std::collections::HashMap;

use druid::Selector;

pub const CMD_GROUPER: Selector<HashMap<String, Vec<String>>> = Selector::new("grouper");
