use std::any::{Any, TypeId};
use std::collections::HashMap;

/// a HashMap with TypeId as key and relative type as value, like async_graphql's data_opt or actix_web::web::Data
/// 同一个struct会因为crate版本不一样而导致typeid不一样h
pub struct TypeIdHashMap {
    inner: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeIdHashMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn insert<T: Any>(&mut self, data: T) {
        let typeid = TypeId::of::<T>();
        self.inner.insert(typeid, Box::new(data));
    }

    /// https://github.com/async-graphql/async-graphql/blob/197fd88409eed8c8dfa698e6602001ed623960ac/src/context.rs#L438-L445
    fn get<T: Any>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T>())
    }
}

#[test]
fn test_typeid_hashmap() {
    let mut map = TypeIdHashMap::new();
    map.insert(0u8);
    map.insert("hello");
    assert_eq!(map.get::<u8>(), Some(&0u8));
    assert_eq!(map.get::<&str>(), Some(&"hello"));
}
