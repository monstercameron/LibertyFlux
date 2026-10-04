// original: 0x00ab95e0 input_ui_slot_table_build (proposed)

/// Build a per-slot lookup table on an input-ui object.
///
/// `this` points to an object holding 11 slots. Each slot has a use count
/// (byte at `+0x2c`), an 8-byte row of emitted values (rows start at
/// `+0x37`, one row per slot), and two flag bytes shared across slots at
/// `+0x98` and `+0xa0` indexed by the inner position. A total counter lives
/// at `+0x8f`.
///
/// Behaviour: ask the count callee for the entry count from `arg0`; a zero
/// count returns 0 at once. Otherwise clear the counts, rows, and total,
/// then for each of the 11 slots and each of 8 inner positions format a name
/// from a global table entry and the position, hash it, and look the hash up.
/// A failed lookup ends the inner positions for that slot. A found entry is
/// set up through one direct call and two calls through its function table
/// (flag bits 0x02 and 0x08 of the answers set the flag bytes), then an
/// emit call produces the row byte while the slot count and total are
/// incremented, and a next-entry call refreshes the entry. Returns the last
/// entry seen.
///
/// Layout notes: the hash call reads the same formatted-name buffer the
/// format call wrote. The format string and the name
/// table are read from the original image. Original is thiscall, one stack
/// word, returns the entry in eax.
lf_checker_rt::export!(thiscall, rw_00ab95e0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 11;
        const POSITIONS: u32 = 8;
        const COUNT_OFF: u32 = 0x2c;
        const ROWS_OFF: u32 = 0x37;
        const TOTAL_OFF: u32 = 0x8f;
        const FLAG_A_OFF: u32 = 0x98;
        const FLAG_B_OFF: u32 = 0xa0;
        const ENTRY_SETUP_ARG_OFF: u32 = 8;
        const ENTRY_SETUP_THIS_OFF: u32 = 0x10;
        const VTABLE_SLOT: u32 = 0x40;
        const NAME_TABLE: u32 = 0x103ee8c;
        const NAME_FMT: u32 = 0xea54b0;
        const CAL_COUNT: u32 = 1;
        const CAL_FORMAT: u32 = 2;
        const CAL_HASH: u32 = 3;
        const CAL_LOOKUP: u32 = 4;
        const CAL_SETUP: u32 = 5;
        const CAL_EMIT: u32 = 7;
        const CAL_NEXT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn inc8(a: u32) {
            unsafe {
                let p = a as *mut u8;
                p.write(p.read().wrapping_add(1));
            }
        }

        let count = lf_checker_rt::callee_cdecl!(CAL_COUNT, u32, arg0);
        if count == 0 {
            return 0;
        }
        for s in 0..SLOTS {
            wr8(this + COUNT_OFF + s, 0);
            ((this + ROWS_OFF + s * POSITIONS) as *mut u64).write_unaligned(0);
        }
        let mut cursor = this + ROWS_OFF;
        let mut saved = count;
        for slot in 0..SLOTS {
            for pos in 0..POSITIONS {
                let mut name = [0u32; 8];
                let entry = rd32(lf_checker_rt::relocated(NAME_TABLE) + slot * 4);
                lf_checker_rt::callee_cdecl!(
                    CAL_FORMAT,
                    u32,
                    name.as_mut_ptr() as u32,
                    lf_checker_rt::relocated(NAME_FMT),
                    entry,
                    pos
                );
                let hash = lf_checker_rt::callee_cdecl!(
                    CAL_HASH,
                    u32,
                    name.as_mut_ptr() as u32,
                    0u32
                );
                let found = lf_checker_rt::callee_stdcall!(CAL_LOOKUP, u32, hash);
                if found == 0 {
                    break;
                }
                let setup_arg = rd32(found + ENTRY_SETUP_ARG_OFF);
                lf_checker_rt::callee_thiscall!(
                    CAL_SETUP,
                    u32,
                    found + ENTRY_SETUP_THIS_OFF,
                    setup_arg
                );
                let vt: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    rd32(rd32(found) + VTABLE_SLOT) as usize,
                );
                if vt(found, 0) & 2 != 0 {
                    wr8(this + pos + FLAG_A_OFF, 1);
                }
                let vt2: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    rd32(rd32(found) + VTABLE_SLOT) as usize,
                );
                if vt2(found, 0) & 8 != 0 {
                    wr8(this + pos + FLAG_B_OFF, 1);
                }
                inc8(this + slot + COUNT_OFF);
                inc8(this + TOTAL_OFF);
                let out =
                    lf_checker_rt::callee_thiscall!(CAL_EMIT, u32, this, slot, pos, arg0)
                        as u8;
                wr8(cursor + pos, out);
                saved = lf_checker_rt::callee_cdecl!(CAL_NEXT, u32, found, saved);
            }
            cursor += POSITIONS;
        }
        saved
    }
});
