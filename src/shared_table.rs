use core::cell::Cell;
use bytemuck::*;
use fxhash::hash64;

pub struct SharedHashTable<T: Clone + Copy + Sized + Send + Sync + NoUninit> {
    inner: Box<[Cell<TableEntry<T>>]>,
}

#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct TableEntry<T: Clone + Copy + Sized + Send + Sync + NoUninit> {
    key: u64,
    hash: u64,
    value: T,
}

unsafe impl<T: Default + Clone + Sized + Send + Sync + NoUninit> Sync for SharedHashTable<T> {}

impl<T: Default + Clone + Sized + Send + Sync + NoUninit> SharedHashTable<T> {
    pub const fn entry_size() -> usize { core::mem::size_of::<TableEntry<T>>() }

    pub fn new(size: usize) -> Self {
        let mut inner = vec![];
        inner.resize_with(size, Cell::default);

        Self { inner: inner.into() }
    }

    pub fn clear(&mut self) {
        for e in self.inner.iter_mut() {
            e.set(TableEntry::default());
        }
    }

    pub fn insert(&self, key: u64, value: T) {
        let hash = hash64(&(key, bytemuck::bytes_of(&value)));
        let entry = TableEntry { key, hash, value };
        self.inner[key as usize % self.inner.len()].set(entry);
    }

    pub fn get(&self, key: u64) -> Option<T> {
        let entry = self.inner[key as usize % self.inner.len()].get();
        let value = entry.value;

        (entry.key == key && entry.hash == hash64(&(key, bytemuck::bytes_of(&value)))).then_some(value)
    }

    pub fn get_place(&self, key: u64) -> Option<T> {
        let entry = self.inner[key as usize % self.inner.len()].get();
        let value = entry.value;

        (entry.hash == hash64(&(key, bytemuck::bytes_of(&value)))).then_some(value)
    }

    pub fn filter_count<F: Fn(T) -> bool>(&self, filter: F) -> usize {
        self.inner.iter().filter(|entry| {
            let entry = entry.get();
            let value = entry.value;

            entry.hash == hash64(&bytemuck::bytes_of(&value)) && filter(value)
        }).count()
    }

    pub fn size(&self) -> usize { self.inner.len() }
}

#[test]
fn test_shared_table() {
    let st = std::sync::Arc::new(SharedHashTable::<usize>::new(10));

    st.insert(0, 123);
    assert_eq!(st.get(0), Some(123));
    st.insert(10, 123);
    assert_eq!(st.get(10), Some(123));
    assert_eq!(st.get(0), None);

    {
        let st = std::sync::Arc::clone(&st);
        std::thread::spawn(move || st.insert(1, 789)).join().unwrap();
    }

    assert_eq!(st.get(1), Some(789));
}
