// original: 0x0069EAE0 reload_active_row
/// Reload the inactive 256-byte row, then flip the active index.
///
/// Reads the object pointer at OBJ and the index at IDX. When the object
/// is null, copies 64 default dwords over row `(idx^1)` of the two-row
/// table at TABLE. Otherwise calls the object's row-read slot (vtable
/// +0x24, three stack words: object, 0x100, row buffer): a zero answer goes
/// straight to the flip; a nonzero answer calls the sync slot (+0x1c, one
/// stack word) and returns without flipping when that is also nonzero,
/// else re-reads the index and object and retries the row read, flipping
/// only when the retry answers zero. The flip xors the index with 1, stores
/// it, and zeroes the first byte of the newly inactive row. Both callees
/// are stdcall (the vtable address sits in ecx across the calls as scratch
/// and is never compared). Returns nothing meaningful (eax is a leftover
/// address or test value on every path), so the contract compares no
/// return value; the row contents and index are observed through the
/// globals comparison. The scripted answers cycle so all four call-history
/// paths run. Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_0069eae0() -> u32 {
    unsafe {
        const OBJ: u32 = 0x018B_7DAC;
        const IDX: u32 = 0x018B_7DA0;
        const TABLE: u32 = 0x018B_7A90;
        const DEFAULTS: u32 = 0x018B_7C90;
        const SLOT_READ: u32 = 0x24;
        const SLOT_SYNC: u32 = 0x1C;
        const ROW_WORDS: usize = 64;
        const ROW_LEN: u32 = 0x100;
        let obj = lf_checker_rt::global::<u32>(OBJ).read_unaligned();
        if obj == 0 {
            let idx = lf_checker_rt::global::<u32>(IDX).read_unaligned();
            let dst = lf_checker_rt::relocated(TABLE).wrapping_add((idx ^ 1).wrapping_shl(8));
            let src = lf_checker_rt::relocated(DEFAULTS);
            core::ptr::copy_nonoverlapping(src as *const u32, dst as *mut u32, ROW_WORDS);
        } else {
            let idx = lf_checker_rt::global::<u32>(IDX).read_unaligned();
            let buf = lf_checker_rt::relocated(TABLE).wrapping_add((idx ^ 1).wrapping_shl(8));
            let vtbl = (obj as *const u32).read_unaligned();
            let f0: extern "stdcall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                ((vtbl.wrapping_add(SLOT_READ)) as *const u32).read_unaligned() as usize,
            );
            let r0 = f0(obj, ROW_LEN, buf);
            if r0 != 0 {
                let obj2 = lf_checker_rt::global::<u32>(OBJ).read_unaligned();
                let vtbl2 = (obj2 as *const u32).read_unaligned();
                let f1: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(
                    ((vtbl2.wrapping_add(SLOT_SYNC)) as *const u32).read_unaligned() as usize,
                );
                let r1 = f1(obj2);
                if r1 != 0 {
                    return 0;
                }
                let idx2 = lf_checker_rt::global::<u32>(IDX).read_unaligned();
                let obj3 = lf_checker_rt::global::<u32>(OBJ).read_unaligned();
                let buf2 =
                    lf_checker_rt::relocated(TABLE).wrapping_add((idx2 ^ 1).wrapping_shl(8));
                let vtbl3 = (obj3 as *const u32).read_unaligned();
                let f0b: extern "stdcall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                    ((vtbl3.wrapping_add(SLOT_READ)) as *const u32).read_unaligned() as usize,
                );
                let r2 = f0b(obj3, ROW_LEN, buf2);
                if r2 != 0 {
                    return 0;
                }
            }
        }
        let idx = lf_checker_rt::global::<u32>(IDX).read_unaligned();
        let new = idx ^ 1;
        lf_checker_rt::global::<u32>(IDX).write_unaligned(new);
        (lf_checker_rt::relocated(TABLE).wrapping_add(new.wrapping_shl(8)) as *mut u8).write(0);
        0
    }
});