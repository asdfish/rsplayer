use {
    phf::phf_map,
    std::collections::HashMap,
    unicode_width::UnicodeWidthStr,
};

const LISTS: [(Option<&str>, (&str, fn() -> Vec<String>)); 1] = [
    (None, ("Print all lists.", || Vec::new()))
];
pub fn hash_map() -> HashMap<Option<&'static str>, (&'static str, fn() -> Vec<String>)> {
    HashMap::from_iter(LISTS.into_iter())
}
