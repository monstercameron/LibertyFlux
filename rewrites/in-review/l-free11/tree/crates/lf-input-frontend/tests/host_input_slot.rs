//! Host tests for the lifted input slots: edge cases a reader of the
//! code would ask about. No rewrites here; the differential proof crate
//! runs the same methods against their verified 32-bit forms.

use lf_input_frontend::input_slot::registry::{counts, ROWS};
use lf_input_frontend::input_slot::{
    Announce, DestroyOutcome, FormatPayload, KeyTable, NotifySinks, NotifyTarget, RegBuild,
    RegEntry, SlotBuild, SlotDrop, SlotLookup, SlotObject, SlotRegistry, SlotRelease, SlotStore,
    ThreadEntry, BIG_SIZE, KIND_OFF, SMALL_SIZE, TABLE_LEN,
};

#[derive(Debug)]
struct Build {
    alloc_ok: bool,
    sizes: Vec<u32>,
    big: [u8; BIG_SIZE],
    small: [u8; SMALL_SIZE],
}

impl SlotBuild for Build {
    type Pending = u32;
    fn alloc(&mut self, size: u32) -> Option<u32> {
        self.sizes.push(size);
        self.alloc_ok.then_some(0xBEEF)
    }
    fn construct_big(&mut self, _block: u32) -> [u8; BIG_SIZE] {
        self.big
    }
    fn small_fill(&mut self, _block: u32) -> [u8; SMALL_SIZE] {
        self.small
    }
}

#[derive(Debug, Default)]
struct Sinks {
    log: Vec<(NotifyTarget, u32)>,
    answer: u32,
}

impl NotifySinks for Sinks {
    fn notify(&mut self, target: NotifyTarget, arg: u32) -> u32 {
        self.log.push((target, arg));
        self.answer
    }
}

#[derive(Debug, Default)]
struct Fmt {
    texts: Vec<u32>,
    next: u32,
}

impl FormatPayload for Fmt {
    fn format(&mut self, _payload: &[u8; 64]) -> u32 {
        self.next += 1;
        self.texts.push(self.next);
        self.next
    }
}

#[derive(Debug, Default)]
struct Emit {
    log: Vec<u32>,
    answer: u32,
}

impl lf_input_frontend::input_slot::AnnounceSink for Emit {
    fn emit(&mut self, text: u32) -> u32 {
        self.log.push(text);
        self.answer
    }
}

fn big_with(pairs: &[(usize, u8)]) -> SlotObject {
    let mut b = [0u8; BIG_SIZE];
    for (off, v) in pairs {
        b[*off] = *v;
    }
    SlotObject::Big(b)
}

fn word_bytes(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

#[test]
fn registry_counts_are_pinned() {
    assert_eq!(ROWS.len(), 14);
    assert_eq!(counts(), (8, 0, 6));
}

#[test]
fn find_free_installs_at_first_empty() {
    let slots = vec![Some(big_with(&[(KIND_OFF, 1)])); 3]
        .into_iter()
        .chain(core::iter::repeat_with(|| None))
        .take(TABLE_LEN as usize)
        .collect();
    let mut store = SlotStore::with_slots(slots, 0);
    let mut build = Build {
        alloc_ok: true,
        sizes: Vec::new(),
        big: [7u8; BIG_SIZE],
        small: [0u8; SMALL_SIZE],
    };
    assert_eq!(store.find_free(true, 0, &mut build), 3);
    assert_eq!(build.sizes, vec![BIG_SIZE as u32]);
    assert_eq!(
        store.slots()[3],
        Some(SlotObject::Big([7u8; BIG_SIZE]))
    );
}

#[test]
fn find_free_small_clears_kind() {
    let mut store = SlotStore::new(0);
    let mut build = Build {
        alloc_ok: true,
        sizes: Vec::new(),
        big: [0u8; BIG_SIZE],
        small: [0xFFu8; SMALL_SIZE],
    };
    assert_eq!(store.find_free(false, 0, &mut build), 0);
    let Some(SlotObject::Small(b)) = &store.slots()[0] else {
        panic!("expected a small record");
    };
    assert_eq!(b[KIND_OFF], 0);
    assert_eq!(b[0], 0xFF, "allocator fill survives around the kind byte");
}

#[test]
fn find_free_full_and_past_end_answer_all_ones() {
    let slots = vec![Some(big_with(&[(KIND_OFF, 1)])); TABLE_LEN as usize];
    let mut store = SlotStore::with_slots(slots, 0);
    let mut build = Build {
        alloc_ok: true,
        sizes: Vec::new(),
        big: [0u8; BIG_SIZE],
        small: [0u8; SMALL_SIZE],
    };
    assert_eq!(store.find_free(true, 0, &mut build), u32::MAX);
    assert!(build.sizes.is_empty(), "no allocation on a full table");
    let mut empty = SlotStore::new(0);
    assert_eq!(store.find_free(true, TABLE_LEN, &mut build), u32::MAX);
    assert_eq!(empty.find_free(false, TABLE_LEN + 1, &mut build), u32::MAX);
}

#[test]
fn find_free_failed_alloc_keeps_cell_empty() {
    let mut store = SlotStore::new(0);
    let mut build = Build {
        alloc_ok: false,
        sizes: Vec::new(),
        big: [0u8; BIG_SIZE],
        small: [0u8; SMALL_SIZE],
    };
    assert_eq!(store.find_free(true, 5, &mut build), 5);
    assert_eq!(store.slots()[5], None);
}

#[test]
fn flag_and_mode_read_direct_and_fallback() {
    let mut rec = [0u8; BIG_SIZE];
    rec[KIND_OFF] = 9;
    rec[0x58] = 0xAB;
    rec[0x54..0x58].copy_from_slice(&word_bytes(0xDEAD_BEEF));
    let mut def = [0u8; BIG_SIZE];
    def[KIND_OFF] = 3;
    def[0x58] = 0x11;
    def[0x54..0x58].copy_from_slice(&word_bytes(0x1234_5678));
    let mut slots = vec![None; TABLE_LEN as usize];
    slots[0] = Some(SlotObject::Big(rec));
    slots[1] = Some(SlotObject::Small([0u8; SMALL_SIZE]));
    slots[7] = Some(SlotObject::Big(def));
    let store = SlotStore::with_slots(slots, 7);
    assert_eq!(store.flag_byte(0), 0xAB);
    assert_eq!(store.mode_word(0), 0xDEAD_BEEF);
    assert_eq!(store.flag_byte(1), 0x11, "unset kind falls back");
    assert_eq!(store.mode_word(1), 0x1234_5678, "unset kind falls back");
}

#[test]
fn notify_selects_sink_or_zero() {
    let mut rec = [0u8; BIG_SIZE];
    rec[KIND_OFF] = 1;
    rec[0x24..0x28].copy_from_slice(&word_bytes(0x5555_AAAA));
    rec[0x48..0x4C].copy_from_slice(&word_bytes(2));
    let mut slots = vec![None; TABLE_LEN as usize];
    slots[0] = Some(SlotObject::Big(rec));
    let store = SlotStore::with_slots(slots, 0);
    let mut sinks = Sinks {
        answer: 0xC0DE,
        ..Sinks::default()
    };
    assert_eq!(store.notify_kind(0, &mut sinks), 0xC0DE);
    assert_eq!(sinks.log, vec![(NotifyTarget::Sink2, 0x5555_AAAA)]);
    // Null slot notifies nothing.
    assert_eq!(store.notify_kind(4, &mut sinks), 0);
    assert_eq!(sinks.log.len(), 1);
    // Unknown kind notifies nothing.
    let mut rec2 = [0u8; BIG_SIZE];
    rec2[KIND_OFF] = 1;
    rec2[0x48..0x4C].copy_from_slice(&word_bytes(9));
    let mut slots2 = vec![None; TABLE_LEN as usize];
    slots2[0] = Some(SlotObject::Big(rec2));
    let store2 = SlotStore::with_slots(slots2, 0);
    assert_eq!(store2.notify_kind(0, &mut sinks), 0);
}

#[test]
fn notify_fallback_uses_all_ones() {
    let mut def = [0u8; BIG_SIZE];
    def[KIND_OFF] = 1;
    def[0x48..0x4C].copy_from_slice(&word_bytes(1));
    let mut slots = vec![None; TABLE_LEN as usize];
    slots[0] = Some(SlotObject::Small([0u8; SMALL_SIZE]));
    slots[6] = Some(SlotObject::Big(def));
    let store = SlotStore::with_slots(slots, 6);
    let mut sinks = Sinks::default();
    assert_eq!(store.notify_kind(0, &mut sinks), 0);
    assert_eq!(sinks.log, vec![(NotifyTarget::Sink1, u32::MAX)]);
}

#[test]
fn announce_inline_and_emitted() {
    let mut rec = [0u8; BIG_SIZE];
    rec[KIND_OFF] = 1;
    rec[0x60..0x64].copy_from_slice(&word_bytes(0xA5A5_A5A5));
    let mut slots = vec![None; TABLE_LEN as usize];
    slots[0] = Some(SlotObject::Big(rec));
    let store = SlotStore::with_slots(slots, 0);
    let mut fmt = Fmt::default();
    let mut emit = Emit {
        answer: 0x99,
        ..Emit::default()
    };
    assert_eq!(store.announce(0, &mut fmt, &mut emit), Announce::Inline);
    assert!(fmt.texts.is_empty() && emit.log.is_empty());
    // Set the flag bit: format then emit.
    let mut rec2 = [0u8; BIG_SIZE];
    rec2[KIND_OFF] = 1;
    rec2[0x20] = 0x40;
    let mut slots2 = vec![None; TABLE_LEN as usize];
    slots2[0] = Some(SlotObject::Big(rec2));
    let store2 = SlotStore::with_slots(slots2, 0);
    assert_eq!(
        store2.announce(0, &mut fmt, &mut emit),
        Announce::Emitted(0x99)
    );
    assert_eq!(fmt.texts, vec![1]);
    assert_eq!(emit.log, vec![1]);
}

#[derive(Debug)]
struct Lookup {
    answer: u32,
    log: Vec<u32>,
}

impl SlotLookup for Lookup {
    fn lookup(&mut self, id: u32) -> u32 {
        self.log.push(id);
        self.answer
    }
}

#[derive(Debug)]
struct Drop {
    rewrite: Option<Option<SlotObject>>,
    log: Vec<u32>,
}

impl SlotDrop for Drop {
    fn drop_slot(&mut self, store: &mut SlotStore, idx: u32) {
        self.log.push(idx);
        if let Some(slot) = self.rewrite.take() {
            store.set_slot(idx, slot);
        }
    }
}

#[derive(Debug)]
struct Release {
    answer: u32,
    saw_none: bool,
    log: Vec<bool>,
}

impl SlotRelease for Release {
    fn release(&mut self, slot: Option<&SlotObject>) -> u32 {
        self.saw_none = slot.is_none();
        self.log.push(!self.saw_none);
        self.answer
    }
}

#[test]
fn destroy_paths() {
    let mk = || {
        let mut rec = [0u8; BIG_SIZE];
        rec[KIND_OFF] = 1;
        rec[0x0C..0x10].copy_from_slice(&word_bytes(0xBEEF_BEEF));
        let mut slots = vec![None; TABLE_LEN as usize];
        slots[2] = Some(SlotObject::Big(rec));
        SlotStore::with_slots(slots, 0)
    };
    // Owned: release runs, answer masked.
    let mut store = mk();
    let mut drop = Drop {
        rewrite: None,
        log: Vec::new(),
    };
    let mut release = Release {
        answer: 0x1234_56FF,
        saw_none: false,
        log: Vec::new(),
    };
    let mut lookup = Lookup {
        answer: 0,
        log: Vec::new(),
    };
    let out = store.destroy(
        2,
        false,
        &ThreadEntry { owned: true },
        &mut lookup,
        &mut drop,
        &mut release,
        );
    assert_eq!(out, DestroyOutcome::Released(0x1234_5600));
    assert_eq!(drop.log, vec![2]);
    assert_eq!(store.slots()[2], None);
    assert!(lookup.log.is_empty());
    // Unowned: no release, slot cleared.
    let mut store = mk();
    let mut release = Release {
        answer: 0,
        saw_none: false,
        log: Vec::new(),
    };
    let out = store.destroy(
        2,
        false,
        &ThreadEntry { owned: false },
        &mut lookup,
        &mut drop,
        &mut release,
        );
    assert_eq!(out, DestroyOutcome::Cleared);
    assert!(release.log.is_empty());
    assert_eq!(store.slots()[2], None);
    // By handle through lookup; negative answers are invalid.
    let mut store = mk();
    lookup.answer = 0xFFFF_FFFF;
    let out = store.destroy(
        0x77,
        true,
        &ThreadEntry { owned: true },
        &mut lookup,
        &mut drop,
        &mut release,
        );
    assert_eq!(out, DestroyOutcome::Invalid);
    assert_eq!(lookup.log.last(), Some(&0x77));
    assert!(store.slots()[2].is_some(), "invalid destroys nothing");
    // Null slot is empty.
    let out = store.destroy(
        9,
        false,
        &ThreadEntry { owned: true },
        &mut lookup,
        &mut drop,
        &mut release,
        );
    assert_eq!(out, DestroyOutcome::Empty);
    // Drop may rewrite the slot; release sees the rewrite.
    let mut store = mk();
    let mut drop = Drop {
        rewrite: Some(None),
        log: Vec::new(),
    };
    let mut release = Release {
        answer: 1,
        saw_none: false,
        log: Vec::new(),
    };
    let out = store.destroy(
        2,
        false,
        &ThreadEntry { owned: true },
        &mut lookup,
        &mut drop,
        &mut release,
        );
    assert_eq!(out, DestroyOutcome::Released(0));
    assert!(release.saw_none);
}

#[test]
fn registry_alloc_success_failure_wrap() {
    #[derive(Debug)]
    struct Reg {
        ok: bool,
        log: Vec<u32>,
    }
    impl RegBuild for Reg {
        type Pending = ();
        fn alloc(&mut self) -> Option<()> {
            self.ok.then_some(())
        }
        fn init(&mut self, _block: (), arg: u32) -> RegEntry {
            self.log.push(arg);
            RegEntry([arg as u8; 8])
        }
    }
    let mut reg = SlotRegistry::new();
    let mut build = Reg { ok: true, log: Vec::new() };
    assert_eq!(reg.alloc_slot(0xAB, &mut build), 0);
    assert_eq!(reg.count(), 1);
    assert_eq!(reg.live(), &[RegEntry([0xAB; 8])]);
    build.ok = false;
    assert_eq!(reg.alloc_slot(0, &mut build), 1);
    assert_eq!(reg.failed(), &[1]);
    // Counter wraps.
    let mut reg = SlotRegistry::with_state(u32::MAX, Vec::new(), Vec::new());
    build.ok = true;
    assert_eq!(reg.alloc_slot(3, &mut build), u32::MAX);
    assert_eq!(reg.count(), 0);
}

#[test]
fn key_table_first_match_or_all_ones() {
    let table = KeyTable {
        ids: [5, 7, 7, 9, 1, 2, 3, 4],
    };
    assert_eq!(table.find(7), 1, "first match wins");
    assert_eq!(table.find(4), 7);
    assert_eq!(table.find(0xFFFF), u32::MAX);
}
