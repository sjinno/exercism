use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map = BTreeMap::<char, i32>::new();
    for (point, letters) in h {
        letters.iter().for_each(|c| {
            map.insert(c.to_ascii_lowercase(), *point);
        });
    }
    map
}
