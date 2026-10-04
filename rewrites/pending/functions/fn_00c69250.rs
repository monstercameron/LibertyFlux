// original: 0x00c69250 gated_count_check
// Return 1 when the id's table object carries 1 at +0x70 and the embedded
// slot array holds at most 2 leading entries, else 0.
export!(thiscall, rw_00c69250(obj: u32, id: u32) -> u32 {
    unsafe {
        let ent = *(relocated(0x1295cd8).wrapping_add(id.wrapping_mul(4)) as *const u32);
        if *((ent as *const u8).add(0x70) as *const u32) != 1 {
            return 0;
        }
        let n: u32 = callee_thiscall!(1, u32, obj);
        if (n as i32) > 2 {
            0
        } else {
            1
        }
    }
});
