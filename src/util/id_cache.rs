use std::collections::HashMap;

pub struct IdCache {
    ids: HashMap<String, usize>,
}

impl Default for IdCache {
    fn default() -> Self { Self::new() }
}

impl IdCache {
    pub fn new() -> Self {
        Self {
            ids: HashMap::default(),
        }
    }

    pub fn len(&self) -> usize { self.ids.len() }

    pub fn is_empty(&self) -> bool { self.ids.is_empty() }

    pub fn get(&self, s: &str) -> usize {
        *self
            .ids
            .get(s)
            .unwrap_or_else(|| panic!("{s} not found in cache"))
    }

    pub fn get_or_add(&mut self, s: String) -> usize {
        let curr_len = self.ids.len();
        *self.ids.entry(s).or_insert(curr_len)
    }
}
