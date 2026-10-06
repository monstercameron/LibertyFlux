// original: 0x006849D0 RC20

/// Rebuild a track set's header, adjust each entry by a range-table lookup,
/// and notify a per-kind callback for every slot.
///
/// `this` points to the set (`+0x00` vtable, `+0x0c` entry array,
/// `+0x10` 16-bit entry count). `source` points to a range table whose
/// first word's two halves sum to the record count; each 12-byte record at
/// `+4` holds a start, a spare field and a length.
///
/// Behaviour: stamp the vtable and clear `+0x04`. When the entry array is
/// non-null, call the source hook (callee 1, thiscall) with the old array
/// and add its answer to the array pointer. Then for each entry index below
/// the count: read the entry; a null entry is left alone; otherwise search
/// the records for the first whose unsigned half-open interval
/// `[start, start+length)` contains the entry (length added wrapping).
/// On a hit add `spare - start` (wrapping) to the entry; on a miss read the
/// worker-context flag byte and, when it is zero, call the log hook
/// (callee 2, cdecl, with the message address and two zeros), then store the
/// entry back unchanged. Finally re-read the (possibly adjusted) entry,
/// call the kind hook (callee 3, stdcall) with the low nibble of the byte at
/// entry `+0x04`, and continue with the reloaded count.
///
/// Comparisons: the entry point uses an unsigned `0 >= count` (taken only
/// for count zero); the range search is unsigned; the record-count,
/// inner-index and outer-index bounds are signed but both sides are never
/// negative, so they behave as equality/upper-bound checks; the
/// found-index-against-minus-one test is an equality that never fires.
/// The function returns `this` and pops one stack word (thiscall/1).
lf_checker_rt::export!(thiscall, rw_006849D0(this: u32, source: u32) -> u32 {
    unsafe {
        const SET_VTABLE: u32 = 0xFE385C;
        const SET_ARRAY: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const LOG_MESSAGE: u32 = 0xFC9C80;
        const REC_LEN: u32 = 12;
        const KIND_MASK: u8 = 0x0f;
        const HOOK_SOURCE: u32 = 1;
        const HOOK_LOG: u32 = 2;
        const HOOK_KIND: u32 = 3;

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

        unsafe fn emit_maybe_log() {
            unsafe {
                let ctx_flag = rd8(rd32(rd32(lf_checker_rt::tls_slot(0)).wrapping_add(4)).wrapping_add(0x0c));
                if ctx_flag == 0 {
                    lf_checker_rt::callee_cdecl!(HOOK_LOG, u32, lf_checker_rt::relocated(LOG_MESSAGE), 0, 0);
                }
            }
        }

        wr32(this, lf_checker_rt::relocated(SET_VTABLE));
        wr32(this.wrapping_add(4), 0);
        let mut array = rd32(this.wrapping_add(SET_ARRAY));
        if array != 0 {
            let shift: u32 = lf_checker_rt::callee_thiscall!(HOOK_SOURCE, u32, source, array);
            array = array.wrapping_add(shift);
            wr32(this.wrapping_add(SET_ARRAY), array);
        }
        let mut index = 0u32;
        if 0u32 >= rd16(this.wrapping_add(SET_COUNT)) {
            return this;
        }
        loop {
            let slot = array.wrapping_add(index.wrapping_mul(4));
            let entry = rd32(slot);
            if entry != 0 {
                let table = rd32(source);
                let records = rd16(table).wrapping_add(rd16(table.wrapping_add(2)));
                let mut hit_delta = 0u32;
                let mut hit = false;
                if (records as i32) > 0 {
                    let mut j = 0u32;
                    let mut rec = table.wrapping_add(4);
                    loop {
                        let start = rd32(rec);
                        if !(entry < start) {
                            let limit = rd32(rec.wrapping_add(8)).wrapping_add(start);
                            if entry < limit {
                                if j == 0xFFFF_FFFF {
                                    break;
                                }
                                hit_delta = rd32(table.wrapping_add(j.wrapping_mul(REC_LEN)).wrapping_add(8))
                                    .wrapping_sub(rd32(table.wrapping_add(j.wrapping_mul(REC_LEN)).wrapping_add(4)));
                                hit = true;
                                break;
                            }
                        }
                        j = j.wrapping_add(1);
                        rec = rec.wrapping_add(REC_LEN);
                        if !((j as i32) < (records as i32)) {
                            break;
                        }
                    }
                }
                if hit {
                    wr32(slot, entry.wrapping_add(hit_delta));
                } else {
                    emit_maybe_log();
                    wr32(slot, entry.wrapping_add(0));
                }
            }
            array = rd32(this.wrapping_add(SET_ARRAY));
            let current = rd32(array.wrapping_add(index.wrapping_mul(4)));
            let kind = (rd8(current.wrapping_add(4)) & KIND_MASK) as u32;
            lf_checker_rt::callee_stdcall!(HOOK_KIND, u32, kind);
            index = index.wrapping_add(1);
            if !((index as i32) < (rd16(this.wrapping_add(SET_COUNT)) as i32)) {
                break;
            }
            array = rd32(this.wrapping_add(SET_ARRAY));
        }
        this
    }
});
