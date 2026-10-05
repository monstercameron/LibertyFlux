// original: 0x00aa0380 stream_open_slot (proposed)

/// Open the slot named `key`, refreshing or allocating its payload, and
/// fill in the caller's fields.
///
/// `found` (third word) is an out flag, cleared first. The slot is located
/// by `key`: a hit with a non-null `wanted` (second word) compares the
/// slot's current generation (two levels down from `[slot + 0x0c]`) against
/// `wanted` through the comparator; on a mismatch the old payload is
/// detached and released and the payload word is cleared. A miss takes a
/// fresh slot from the free list instead, failing null when exhausted. An
/// empty payload word with a non-null `wanted` is allocated through the
/// allocator; a null allocation resets the key word, detaches the slot and
/// fails null. Otherwise the payload is attached, the slot's key word,
/// float, tag and marker byte take `key`, `weight`, `0` and the low byte of
/// `flags`, the payload word is marked present, `found` is set, and the
/// payload word is the result.
///
/// Original: 0x00aa0380 (thiscall, five stack words).
lf_checker_rt::export!(
    thiscall,
    rw_00aa0380(this: u32, key: u32, wanted: u32, found: u32, weight: u32, flags: u32) -> u32 {
        unsafe {
            const PAYLOAD_OFF: u32 = 0x0c;
            const GEN_OFF: u32 = 0x78;
            const GEN_TAG_OFF: u32 = 0xf8;
            const KEY_OFF: u32 = 8;
            const WEIGHT_OFF: u32 = 0x10;
            const TAG_OFF: u32 = 0x14;
            const PRESENT_OFF: u32 = 0x18;
            const MARKER_OFF: u32 = 0x19;
            const FIND: u32 = 1;
            const COMPARE: u32 = 2;
            const DETACH_PAYLOAD: u32 = 3;
            const RELEASE_PAYLOAD: u32 = 4;
            const TAKE_FRESH: u32 = 5;
            const ALLOC: u32 = 6;
            const ATTACH: u32 = 7;
            const DETACH_SLOT: u32 = 8;
            (found as *mut u8).write(0);
            let mut slot: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, this, key);
            if slot != 0 {
                if wanted != 0 {
                    let inner = (((slot + PAYLOAD_OFF) as *const u32).read_unaligned()
                        + GEN_OFF) as *const u32;
                    let gen_obj = inner.read_unaligned();
                    let gen_tag =
                        ((gen_obj + GEN_TAG_OFF) as *const u32).read_unaligned();
                    let same: u32 =
                        lf_checker_rt::callee_cdecl!(COMPARE, u32, gen_tag, 0);
                    if same != wanted {
                        let payload =
                            ((slot + PAYLOAD_OFF) as *const u32).read_unaligned();
                        lf_checker_rt::callee_thiscall!(DETACH_PAYLOAD, u32, this, payload);
                        lf_checker_rt::callee_thiscall!(RELEASE_PAYLOAD, u32, this, payload);
                        ((slot + PAYLOAD_OFF) as *mut u32).write_unaligned(0);
                    }
                }
            } else {
                slot = lf_checker_rt::callee_thiscall!(TAKE_FRESH, u32, this);
                if slot == 0 {
                    return 0;
                }
            }
            if ((slot + PAYLOAD_OFF) as *const u32).read_unaligned() == 0 {
                if wanted != 0 {
                    let made: u32 =
                        lf_checker_rt::callee_thiscall!(ALLOC, u32, this, wanted, 0, 0);
                    ((slot + PAYLOAD_OFF) as *mut u32).write_unaligned(made);
                }
                let payload = ((slot + PAYLOAD_OFF) as *const u32).read_unaligned();
                if payload == 0 {
                    ((slot + KEY_OFF) as *mut u32).write_unaligned(0);
                    lf_checker_rt::callee_thiscall!(DETACH_SLOT, u32, this, slot);
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(ATTACH, u32, this, payload);
                ((slot + KEY_OFF) as *mut u32).write_unaligned(key);
                ((slot + WEIGHT_OFF) as *mut u32).write_unaligned(weight);
                ((slot + TAG_OFF) as *mut u32).write_unaligned(0);
                ((slot + MARKER_OFF) as *mut u8).write(flags as u8);
                (found as *mut u8).write(1);
            }
            let payload = ((slot + PAYLOAD_OFF) as *const u32).read_unaligned();
            ((slot + PRESENT_OFF) as *mut u8).write(1);
            payload
        }
    }
);
