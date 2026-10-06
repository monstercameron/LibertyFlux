// original: 0x00d6be30 filemem_entry_insert (proposed)

/// Insert a new entry into the file-memory table, or reject a duplicate.
///
/// `this` is the file-memory context (sub-object at `+4`, entry table at
/// `+0x9c` with an array pointer at `+0` and a 16-bit count at `+4`; each
/// entry carries its key at `+0x14`). The four stack words are three flag
/// bytes (`create`, `wide`, `force`, of which only the low byte is read)
/// and a `want` key. Returns 1 on success, 0 when the key is already
/// present. Only the low byte of the return is defined.
///
/// Behaviour: unless `force` is zero, set byte `+0x18` of the sub-object.
/// Take the allocator's current key (callee 1); `want`, when nonzero,
/// replaces it. Scan the table: each entry whose key equals the wanted key
/// or lies one below or above it (wrapping; the below-test is skipped for a
/// zero key) rejects the insert with 0. Otherwise allocate a block (callee
/// 2, failing to a null object), construct the entry (callee 3), stamp the
/// wanted key at `+0x14`, and store the new entry through the slot the
/// table helper (callee 4) returns; a sizing helper (callee 5) sees the
/// array, the count, 4 and a constant address. Re-scan for the new entry:
/// its index (or -1) is stored at `+0xa0` and `+0xf8`, with -1 also stored
/// at `+0xfc`. A positive index whose predecessor exists and is non-null
/// copies that entry's fields into the new one: bytes `+0`, `+4` (then, for
/// a nonzero `want`, kind 10 becomes 8 with flag 0; kinds 9 and 10 set flag
/// `+4` to 1, anything else to 0), bytes `+8`, `+1`, `+2`, bit 2 of byte
/// `+3`, dwords `+0x20`, `+0x24`, `+0x28`, `+0x30`, `+0x2c`, bytes `+9`,
/// `+0xa`, plus byte `+0xb` and dword `+0x10` only when `wide` is nonzero,
/// plus dword `+0x48` only for kinds 9 and 10. Then a notifier runs (callee
/// 6), bit 7 of the found entry's `+0xc` word is set, byte `+0x108` of the
/// context is cleared, and, unless `create` is zero, a checker runs (callee
/// 7): a zero low byte calls a repair helper (callee 8) with 1, and for a
/// nonzero `want` with bit 0 set on sub-object byte `+0x1b` that bit is
/// cleared. Return 1.
///
/// The index comparisons are SIGNED (`count <= 0` misses, `index <= 0`
/// skips the copy); the predecessor bound (`index - 1 >= count`) is
/// UNSIGNED; the repair test looks only at the LOW byte of its answer.
///
/// Original: 0x00d6be30 (thiscall, ECX = this, four stack words; callees 1,
/// 3, 6 and 7 take no stack words, callee 4 and 8 take one, callee 2 takes
/// one and callee 5 takes four, both cdecl).
lf_checker_rt::export!(thiscall, rw_00d6be30(this: u32, create: u32, wide: u32, want: u32, force: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x04;
        const TABLE_OFF: u32 = 0x9C;
        const INDEX_OFF: u32 = 0xA0;
        const CUR_OFF: u32 = 0xF8;
        const PREV_OFF: u32 = 0xFC;
        const DONE_OFF: u32 = 0x108;
        const ARR_OFF: u32 = 0x00;
        const COUNT_OFF: u32 = 0x04;
        const ENTRY_KEY: u32 = 0x14;
        const CAL_NEWKEY: u32 = 1;
        const CAL_ALLOC: u32 = 2;
        const CAL_CTOR: u32 = 3;
        const CAL_SLOT: u32 = 4;
        const CAL_SIZE: u32 = 5;
        const CAL_NOTIFY: u32 = 6;
        const CAL_CHECK: u32 = 7;
        const CAL_REPAIR: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let sub = rd32(this.wrapping_add(SUB_OFF));
        if (force as u8) != 0 {
            wr8(sub.wrapping_add(0x18), 1);
        }
        let t: u32 = lf_checker_rt::callee_thiscall!(CAL_NEWKEY, u32, this);
        let table = rd32(this.wrapping_add(TABLE_OFF));
        let count = rd16(table.wrapping_add(COUNT_OFF));
        let array = rd32(table.wrapping_add(ARR_OFF));
        let mut key = t;
        if want != 0 {
            key = want;
        }
        let end = array.wrapping_add(count.wrapping_mul(4));
        let mut slot = array;
        while slot != end {
            let c = rd32(rd32(slot).wrapping_add(ENTRY_KEY));
            if c == key || c.wrapping_add(1) == key || (c != 0 && c.wrapping_sub(1) == key) {
                return 0;
            }
            slot = slot.wrapping_add(4);
        }
        let m: u32 = lf_checker_rt::callee_cdecl!(CAL_ALLOC, u32, 0x50);
        let new: u32 = if m == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CAL_CTOR, u32, m)
        };
        wr32(new.wrapping_add(ENTRY_KEY), key);
        let table2 = rd32(this.wrapping_add(TABLE_OFF));
        let slotptr: u32 = lf_checker_rt::callee_thiscall!(CAL_SLOT, u32, table2, 0x10);
        wr32(slotptr, new);
        let table3 = rd32(this.wrapping_add(TABLE_OFF));
        let count2 = rd16(table3.wrapping_add(COUNT_OFF));
        // The original branches on the sign of this zero-extended count;
        // it is never negative, so the count always goes to the helper.
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CAL_SIZE, u32, rd32(table3.wrapping_add(ARR_OFF)), count2, 4, lf_checker_rt::relocated(0x00D708D0)
        );
        let table4 = rd32(this.wrapping_add(TABLE_OFF));
        let count3 = rd16(table4.wrapping_add(COUNT_OFF));
        let array4 = rd32(table4.wrapping_add(ARR_OFF));
        let mut idx: i32 = -1;
        if count3 != 0 {
            let mut i: u32 = 0;
            while i < count3 {
                if rd32(array4.wrapping_add(i.wrapping_mul(4))) == new {
                    idx = i as i32;
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        wr32(this.wrapping_add(INDEX_OFF), idx as u32);
        wr32(this.wrapping_add(CUR_OFF), idx as u32);
        wr32(this.wrapping_add(PREV_OFF), 0xFFFFFFFF);
        if idx > 0 {
            // The original's sign check on (index - 1) here is dead: the
            // index is already known positive.
            let under = (idx as u32).wrapping_sub(1);
            let count4 = rd16(table4.wrapping_add(COUNT_OFF));
            if under < count4 {
                let prev = rd32(array4.wrapping_add((idx as u32).wrapping_mul(4)).wrapping_sub(4));
                if prev != 0 {
                    wr8(new.wrapping_add(0x00), rd8(prev.wrapping_add(0x00)));
                    wr8(new.wrapping_add(0x04), rd8(prev.wrapping_add(0x04)));
                    if want != 0 && rd8(new.wrapping_add(0x00)).wrapping_sub(1) == 9 {
                        wr8(new.wrapping_add(0x00), 8);
                        wr8(new.wrapping_add(0x04), 0);
                    }
                    let kind1 = (rd8(new.wrapping_add(0x00)) as u32).wrapping_sub(1);
                    if kind1 == 8 || kind1 == 9 {
                        wr8(new.wrapping_add(0x04), 1);
                    } else {
                        wr8(new.wrapping_add(0x04), 0);
                    }
                    wr8(new.wrapping_add(0x08), rd8(prev.wrapping_add(0x08)));
                    wr8(new.wrapping_add(0x01), rd8(prev.wrapping_add(0x01)));
                    wr8(new.wrapping_add(0x02), rd8(prev.wrapping_add(0x02)));
                    let b3 = rd8(prev.wrapping_add(0x03)) ^ rd8(new.wrapping_add(0x03));
                    wr8(new.wrapping_add(0x03), rd8(new.wrapping_add(0x03)) ^ (b3 & 4));
                    if (wide as u8) != 0 {
                        wr8(new.wrapping_add(0x0B), rd8(prev.wrapping_add(0x0B)));
                        wr32(new.wrapping_add(0x10), rd32(prev.wrapping_add(0x10)));
                    }
                    wr32(new.wrapping_add(0x20), rd32(prev.wrapping_add(0x20)));
                    wr32(new.wrapping_add(0x24), rd32(prev.wrapping_add(0x24)));
                    wr8(new.wrapping_add(0x09), rd8(prev.wrapping_add(0x09)));
                    wr8(new.wrapping_add(0x0A), rd8(prev.wrapping_add(0x0A)));
                    wr32(new.wrapping_add(0x28), rd32(prev.wrapping_add(0x28)));
                    wr32(new.wrapping_add(0x30), rd32(prev.wrapping_add(0x30)));
                    wr32(new.wrapping_add(0x2C), rd32(prev.wrapping_add(0x2C)));
                    if kind1 == 9 || kind1 == 8 {
                        wr32(new.wrapping_add(0x48), rd32(prev.wrapping_add(0x48)));
                    }
                }
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, sub);
        let table5 = rd32(this.wrapping_add(TABLE_OFF));
        let idx5 = rd32(this.wrapping_add(INDEX_OFF));
        let found = rd32(rd32(table5.wrapping_add(ARR_OFF)).wrapping_add(idx5.wrapping_mul(4)));
        wr32(found.wrapping_add(0x0C), rd32(found.wrapping_add(0x0C)) | 0x80);
        wr8(this.wrapping_add(DONE_OFF), 0);
        if (create as u8) != 0 {
            let t2: u32 = lf_checker_rt::callee_thiscall!(CAL_CHECK, u32, sub);
            if (t2 & 0xFF) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_REPAIR, u32, sub, 1);
            }
            if want != 0 {
                let b = rd8(sub.wrapping_add(0x1B));
                if ((!b as u32) & 1) == 0 {
                    wr8(sub.wrapping_add(0x1B), b & 0xFE);
                }
            }
        }
        1
    }
});
