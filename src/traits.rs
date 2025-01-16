pub trait Cache<K, V> {

    // Inserts a value
    fn put(&mut self, key: K, value: V);

    // Retrieves a value associated to a key
    fn get(&mut self, k: &K) -> Option<&V>;
}