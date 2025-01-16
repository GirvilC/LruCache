//! # LRU Cache
//! 
//! Cette application permet l'utilisation d'un cache LRU via le langage Rust
//! Fonctionnalité supporté :
//! - Gestion d'un dictionnaire clé / valeur générique
//! - Utilisation d'un trait
//! 
//! # Exemple d'utilisation
//! ```rust
//! 
//! use lru_cache::LruCache;
//! 
//! let mut cache = LruCache::new(3);
//! cache.put("A", String::from("value_a"));
//! cache.put("B", String::from("value_b"));
//! cache.put("C", String::from("value_c"));
//! cache.put("D", String::from("value_d")); // Supprime A
//! assert_eq!(cache.get(&"A"), None);
//! assert_eq!(cache.get(&"B"), Some(&String::from("value_b"));
//! 
//! ```





mod traits;
mod testcache;
mod lrucache;



fn main() {

}