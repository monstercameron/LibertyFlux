//! The indexed-list check of the layout frame.
//!
//! A frame can resolve every item of an indexed list when the frame answers
//! its first query with a non-zero low byte, the shared registry has a record
//! for the frame's lookup key, that record names a list, and every entry of
//! the list has a key the shared resolver accepts. When any step fails the
//! check answers the frame's own list field, which is the fallback; when every
//! step passes it answers the list.
//!
//! The registry, the list records and the entries are shared memory the frame
//! does not own, so the method takes an [`IndexedListHost`] for them. The
//! answer is either the fallback field of the frame or the list handle.

/// The answer of the check: the frame's own list field (the fallback), or the
/// registry's list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListAnswer {
    /// The frame's own list field at the fallback offset. A step failed, or
    /// the registry has nothing for the frame.
    FieldFallback,
    /// The registry's list, every entry of which resolves. The handle is
    /// opaque to the lift.
    List(u32),
}

/// The calls the check makes outside the frame's bytes.
pub trait IndexedListHost {
    /// The frame's first query. Only its low byte matters to the check.
    fn first_query(&mut self, frame: &mut super::frame::UILayoutFrame) -> u32;

    /// The frame's second query: the key the registry is looked up with.
    fn second_query(&mut self, frame: &mut super::frame::UILayoutFrame) -> u32;

    /// The registry record for `key`, or zero when there is none.
    fn registry_record(&mut self, key: u32) -> u32;

    /// The list that a registry record names, or zero when it names none.
    fn registry_list(&mut self, record: u32) -> u32;

    /// The number of entries in a list.
    fn list_count(&mut self, list: u32) -> u16;

    /// The entry at `index` of a list. The index is below the count.
    fn list_entry(&mut self, list: u32, index: usize) -> u32;

    /// The key an entry carries.
    fn entry_key(&mut self, entry: u32) -> u32;

    /// The shared resolver's answer for an entry key. Only its low byte matters.
    fn key_resolves(&mut self, key: u32) -> u32;
}

impl super::frame::UILayoutFrame {
    /// Checks whether the frame can resolve every item of its indexed list.
    /// Each failing step answers the frame's fallback field; a passing check
    /// answers the list. An empty list passes.
    pub fn every_list_entry_resolves<H: IndexedListHost>(&mut self, host: &mut H) -> ListAnswer {
        if host.first_query(self) as u8 == 0 {
            return ListAnswer::FieldFallback;
        }
        let key = host.second_query(self);
        let record = host.registry_record(key);
        if record == 0 {
            return ListAnswer::FieldFallback;
        }
        let list = host.registry_list(record);
        if list == 0 {
            return ListAnswer::FieldFallback;
        }
        let count = host.list_count(list);
        for index in 0..usize::from(count) {
            let entry = host.list_entry(list, index);
            let entry_key = host.entry_key(entry);
            if host.key_resolves(entry_key) as u8 == 0 {
                return ListAnswer::FieldFallback;
            }
        }
        ListAnswer::List(list)
    }
}
