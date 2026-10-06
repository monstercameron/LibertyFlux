
// original: 0x00955860 count_valid_entries (proposed)
/// Count the valid entries of a record by probing each through its vtable.
///
/// Returns 0 when the signed count at `COUNT_OFF` (0xF8C) is not
/// positive. Otherwise, per entry: calls the slot-`VT_SLOT_A` method; a
/// null answer falls back to the pointer at `FALLBACK_OFF` (0x100), while
/// a non-null answer is probed again and the result's slot-`VT_SLOT_B`
/// method is called. A null object skips the entry. Otherwise the entry
/// at `ENTRIES_OFF` (0xF88) plus index times `ENTRY_STRIDE` (0x34) is
/// read: entries whose flag dword at `FLAG_OFF` (0x10), shifted right 8,
/// has bit 0 set are skipped; the rest look up a SIGNED dword from the
/// key-selected table (`KEY_TABLE` indexed by the sign-extended key at
/// `KEY_OFF` (0x2E); the dword at `SUB_TABLE_OFF` (0xCC) in that row
/// points at the word array indexed by entry+0) and count the entry when
/// it is at least 0 (SIGNED: compare against -1, skip when below or
/// equal). The two
/// `i >= count` guards in the original are dead (the loop condition is
/// `i < count` with a stable count), reproduced here as structured
/// branches that skip instead of faulting. Original is cdecl/1,
/// returns EAX.
lf_checker_rt::export!(cdecl, rw_00955860(rec: u32) -> u32 {
    const COUNT_OFF: u32 = 0xF8C;
    const ENTRIES_OFF: u32 = 0xF88;
    const FALLBACK_OFF: u32 = 0x100;
    const KEY_OFF: u32 = 0x2E;
    const ENTRY_STRIDE: u32 = 0x34;
    const FLAG_OFF: u32 = 0x10;
    const KEY_TABLE: u32 = 0x01295CD8;
    const SUB_TABLE_OFF: u32 = 0xCC;
    const VT_SLOT_A: u32 = 0xA0;
    const VT_SLOT_B: u32 = 0xE0;
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe {
        let count = rd32(rec.wrapping_add(COUNT_OFF)) as i32;
        if count <= 0 {
            return 0;
        }
        let entries = rd32(rec.wrapping_add(ENTRIES_OFF));
        let mut n: u32 = 0;
        let mut i: i32 = 0;
        while i < count {
            let vt = rd32(rec);
            let probe_a: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_A)) as usize);
            let first = probe_a(rec);
            let obj = if first == 0 {
                rd32(rec.wrapping_add(FALLBACK_OFF))
            } else {
                let second = probe_a(rec);
                let vt2 = rd32(second);
                let probe_b: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt2.wrapping_add(VT_SLOT_B)) as usize);
                probe_b(second)
            };
            if obj != 0 {
                // Dead-by-construction guards (see doc): skip if taken.
                let p = if i < count {
                    entries.wrapping_add((i as u32).wrapping_mul(ENTRY_STRIDE))
                } else {
                    0
                };
                if p != 0 {
                    let flags = rd32(p.wrapping_add(FLAG_OFF));
                    if (flags >> 8) & 1 == 0 {
                        let q = if i < count {
                            entries.wrapping_add((i as u32).wrapping_mul(ENTRY_STRIDE))
                        } else {
                            0
                        };
                        if q != 0 {
                            let idx = rd32(q);
                            let key = (rec.wrapping_add(KEY_OFF) as *const u16).read_unaligned()
                                as i16 as i32;
                            let slot = lf_checker_rt::relocated(KEY_TABLE)
                                .wrapping_add((key as u32).wrapping_mul(4));
                            let arr = rd32(rd32(slot).wrapping_add(SUB_TABLE_OFF));
                            let v = rd32(arr.wrapping_add(idx.wrapping_mul(4))) as i32;
                            if v >= 0 {
                                n = n.wrapping_add(1);
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        n
    }
});
