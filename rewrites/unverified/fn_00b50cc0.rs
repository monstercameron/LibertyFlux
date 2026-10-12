// original: 0x00B50CC0 event_group_contains_related

/// Scan the group's signed count of event pointers from byte offset `0x0c`.
/// Skip null entries and entries equal to the supplied event pointer. For every
/// other event, call vtable slot `0x38` with the supplied event as its single
/// stack argument; return byte one on the first true result, otherwise zero.
/// The method is thiscall and pops its pointer argument.
lf_checker_rt::export!(thiscall, rw_contains_related(group: *const u8, other_event: u32) -> u8 {
    const COUNT_OFFSET: usize = 0x08;
    const EVENTS_OFFSET: usize = 0x0c;
    const EVENT_POINTER_BYTES: usize = 4;
    const RELATED_SLOT_OFFSET: usize = 0x38;
    unsafe {
        let count = (group.add(COUNT_OFFSET) as *const i32).read_unaligned();
        if count <= 0 { return 0; }
        for index in 0..(count as usize) {
            let event = (group.add(EVENTS_OFFSET + index * EVENT_POINTER_BYTES) as *const u32).read_unaligned();
            if event == 0 || event == other_event { continue; }
            let vtable = (event as *const u32).read_unaligned();
            let matcher_address = ((vtable as *const u8).add(RELATED_SLOT_OFFSET) as *const u32).read_unaligned();
            let matcher: extern "thiscall" fn(u32, u32) -> u8 = core::mem::transmute(matcher_address as usize);
            let matches = matcher(event, other_event) != 0;
            if matches { return 1; }
        }
        0
    }
});
