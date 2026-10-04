// original: 0x00c66f80 cutscene_resolve_and_clear_record
/// Resolves a record through gate callee 1 and clears twelve of its words.
///
/// The gate answers through an out-slot plus an al status; a zero status
/// ends the call. Otherwise the out-slot's low word tags helper callee 2,
/// whose record gets zeroed at the fixed offsets.
export!(thiscall, rw_c66f80(this: u32, arg: u32) -> u32 {
    let gate = unsafe { (this as *const u32).byte_add(0x34).read() };
    let table = unsafe { (gate as *const u32).read() };
    let pick = unsafe { (table as *const u32).byte_add(0xc).read() };
    let mut out_slot: u32 = 0;
    let status =
        callee_thiscall!(1, u32, pick, arg, &mut out_slot as *mut u32 as u32);
    if (status & 0xFF) == 0 {
        return status;
    }
    let record = callee_thiscall!(2, u32, this, out_slot & 0xFFFF);
    for off in [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38] {
        unsafe { ((record as *mut u32).byte_add(off)).write(0) };
    }
    record
});
