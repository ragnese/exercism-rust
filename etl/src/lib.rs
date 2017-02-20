use std::collections::BTreeMap;

pub fn transform(old: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    old.iter()
        .flat_map(|(key, vec)| vec.iter().map(move |x| (x.to_lowercase(), key.clone())))
        .collect()
}
