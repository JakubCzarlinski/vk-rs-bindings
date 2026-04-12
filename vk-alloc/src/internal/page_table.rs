use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct PageTable<K, V> {
    pages: BTreeMap<K, V>,
}

impl<K, V> Default for PageTable<K, V> {
    fn default() -> Self {
        Self {
            pages: BTreeMap::new(),
        }
    }
}

impl<K, V> PageTable<K, V>
where
    K: Ord,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.pages.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.pages.remove(key)
    }

    #[cfg_attr(not(any(test, feature = "bench-internals")), allow(dead_code))]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.pages.get(key)
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        for (key, value) in &self.pages {
            f(key, value);
        }
    }
}
