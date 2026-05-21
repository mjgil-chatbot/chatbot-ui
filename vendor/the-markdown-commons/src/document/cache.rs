use std::collections::{HashMap, VecDeque};
const DOCUMENT_CACHE_MAX_ENTRIES: usize = 128;
const DOCUMENT_CACHE_MAX_BYTES: usize = 8 * 1024 * 1024;
pub(super) fn touch_cache_order(order: &mut VecDeque<String>, input: &str) {
    order.retain(|candidate| candidate != input);
    order.push_back(input.to_string());
}
