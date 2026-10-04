// original: 0x00BE82E0 record_clear_on_match (proposed)
/// Clear a record when its id word equals the given id.
///
/// `obj` points to the record, `id` is the expected id. The id word at
/// `+0x00` is compared against `id`; on equality the id word is zeroed,
/// the byte at `+0x04` is zeroed and the word at `+0x3e` is set to
/// 0x0100, otherwise nothing is written. Returns the id word as found.
///
/// Original: 0x00BE82E0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00BE82E0(obj: u32, id: u32) -> u32 {
    unsafe {
        const ID: u32 = 0x00;
        const STATE: u32 = 0x04;
        const KIND: u32 = 0x3e;
        const CLEARED_KIND: u16 = 0x0100;
        let found = ((obj + ID) as *const u32).read_unaligned();
        if found == id {
            ((obj + ID) as *mut u32).write_unaligned(0);
            ((obj + STATE) as *mut u8).write(0);
            ((obj + KIND) as *mut u16).write_unaligned(CLEARED_KIND);
        }
        found
    }
});

