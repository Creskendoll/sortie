use std::{collections::HashMap, sync::Arc};

use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub selected_path: String,
    // https://docs.rs/druid/0.8.2/druid/trait.Data.html
    pub file_groups: Arc<HashMap<String, Vec<String>>>,
}

pub fn initial_data() -> AppState {
    AppState {
        selected_path: String::new(),
        file_groups: Arc::new(HashMap::new()),
    }
}
