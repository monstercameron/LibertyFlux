// original: 0x00c2dd10 gated_table_lookup
/// Gated lookup through the membership test.
///
/// After the enable-bit and index-range gates, runs the membership test
/// as a subroutine; when it passes, looks up the index in the object's
/// table and returns whether the slot is nonzero.
export!(thiscall, rw_00c2dd10(this: *const u8, arg: u32) -> u32 {
    unsafe {
        if (((( *(this.add(0x10) as *const u32)) >> 0x12) as u8) & 1) == 0 {
            return 0;
        }
        let cl = *this.add(4);
        if (cl as i8) < 0 {
            return 0;
        }
        let edi = arg;
        let ok: u32 = callee_thiscall!(1, u32, this as u32, edi);
        if (ok as u8) == 0 {
            return 0;
        }
        let l1 = *((edi.wrapping_add(0xDC4)) as *const u32);
        let l2 = *((l1.wrapping_add(0x64)) as *const u32);
        if l2 == 0 {
            return 0;
        }
        let table = *((l2.wrapping_add(0x1B0)) as *const u32);
        if table == 0 {
            return 0;
        }
        let idx = (cl as i8 as i32) as u32;
        if (*(table.wrapping_add(idx * 4) as *const u32)) == 0 {
            return 0;
        }
        1
    }
});
