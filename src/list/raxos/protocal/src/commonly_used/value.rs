use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::Value;

impl Value for String {}

impl<K, V> Value for BTreeMap<K, V>
where
    K: Debug + Clone + Send + 'static,
    V: Debug + Clone + Send + 'static,
{
}
