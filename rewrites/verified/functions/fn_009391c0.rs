// original: 0x009391C0 stream_slot_field_get (proposed)

/// Load the tagged field of the slot linked from a streaming object.
///
/// `this` points to the object; dword at `+0x17D0` is a pointer to a slot
/// record or null. When non-null, returns the dword at slot `+0x0A`
/// (unaligned); when null, returns 0. Reads only (thiscall).
lf_checker_rt::export!(thiscall, rw_009391c0(this: u32) -> u32 {
    unsafe {
        const SLOT_LINK: u32 = 0x17D0;
        const TAG_FIELD: u32 = 0x0A;
        let slot = ((this + SLOT_LINK) as *const u32).read_unaligned();
        if slot == 0 {
            0
        } else {
            ((slot + TAG_FIELD) as *const u32).read_unaligned()
        }
    }
});
