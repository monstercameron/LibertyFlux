// original: 0x00cc55e0 euphoria_behaviour_find_or_create
//! Behaviour-registry lookup with create-on-miss (see crate docs for the
//! checker runtime used for addresses and intercepted callees).
/// File VA of the head pointer of the behaviour-registry list.
const REGISTRY_HEAD: u32 = 0x171c0f8;
/// Allocation size of one behaviour record, in bytes.
const RECORD_SIZE: u32 = 0x37c;
/// Allocation size of one registry list node: [record, next].
const NODE_SIZE: u32 = 8;
/// Byte offset of the flags word inside a behaviour record.
const FLAGS_OFF: u32 = 0x378;
/// Record flag tested and cleared when an existing record is re-acquired.
const FLAG_REFRESH: u32 = 0x1000;
/// Byte offset where the channel table starts inside a fresh record.
const CHANNELS_OFF: u32 = 8;
/// Size of one channel slot: (id, enabled, 0, 0, 0, 0).
const CHANNEL_STRIDE: u32 = 24;

/// Default channel table written into every new record: (channel id, enabled).
const DEFAULT_CHANNELS: [(u32, u32); 35] = [
    (0x0f, 1), (0x0f, 1), (0x0f, 1), (0x0f, 0), (0x0f, 1), (0x0f, 1), (0x0f, 1),
    (0x15, 1), (0x15, 0), (0x13, 0), (0x12, 0), (0x14, 0), (0x16, 0), (0x16, 1),
    (0x24, 0), (0x22, 0), (0x20, 0), (0x1c, 0), (0x21, 0), (0x23, 0), (0x25, 0),
    (0x2e, 1), (0x2e, 0), (0x2c, 0), (0x28, 0), (0x2d, 0), (0x2f, 0), (0x2f, 1),
    (0x33, 1), (0x33, 1), (0x33, 0), (0x32, 0), (0x34, 0), (0x34, 1), (0x34, 1),
];

/// Find the behaviour record tagged with `key`, creating and registering a
/// fresh default-initialised record when none exists.
///
/// The registry is a singly linked list of two-word nodes; each node points
/// at a record whose first word is its tag. A re-acquired record whose refresh
/// flag is set is passed through the refresh helper before the flag is
/// cleared. Both allocation-failure paths fall through to a store through the
/// null pointer, faulting exactly like the original.
export!(cdecl, rw_cc55e0(key: u32) -> u32 {
    unsafe {
        let mut node = *(relocated(REGISTRY_HEAD) as *const u32);
        while node != 0 {
            let rec = *(node as *const u32);
            if *(rec as *const u32) == key {
                let flags = (rec + FLAGS_OFF) as *mut u32;
                if *flags & FLAG_REFRESH != 0 {
                    let _: u32 = callee_cdecl!(3, u32, rec);
                    let rec = *(node as *const u32);
                    *((rec + FLAGS_OFF) as *mut u32) &= !FLAG_REFRESH;
                }
                return *(node as *const u32);
            }
            node = *((node + 4) as *const u32);
        }
        let blk: u32 = callee_cdecl!(1, u32, RECORD_SIZE);
        if blk == 0 {
            core::ptr::write_volatile(0 as *mut u32, key);
            return 0;
        }
        let rec: u32 = callee_thiscall!(2, u32, blk);
        *(rec as *mut u32) = key;
        let list_node: u32 = callee_cdecl!(4, u32, NODE_SIZE);
        if list_node == 0 {
            core::ptr::write_volatile(0 as *mut u32, rec);
            return 0;
        }
        *((list_node + 4) as *mut u32) = 0;
        *(list_node as *mut u32) = rec;
        let _: u32 = callee_thiscall!(5, u32, relocated(REGISTRY_HEAD), list_node);
        for (i, &(id, enabled)) in DEFAULT_CHANNELS.iter().enumerate() {
            let slot = (rec + CHANNELS_OFF + (i as u32) * CHANNEL_STRIDE) as *mut u32;
            *slot = id;
            *slot.add(1) = enabled;
            *slot.add(2) = 0;
            *slot.add(3) = 0;
            *slot.add(4) = 0;
            *slot.add(5) = 0;
        }
        let _: u32 = callee_cdecl!(3, u32, rec);
        rec
    }
});
