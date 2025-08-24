use std::hash::BuildHasherDefault;
use indexmap::IndexMap;
use rustc_hash::FxHasher;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

macro_rules! map {
    () => { FxIndexMap::default() };
}

// impl FxIndexMap<K, V> {
//     fn from_vec
// }

#[test]
fn test_map() {
    let mut map = map!();
    map.insert("test", 0);
    let mut map2 = FxIndexMap::default();
    map2.insert("test", 0);
    assert_eq!(map, map2)
}