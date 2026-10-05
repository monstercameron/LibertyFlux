// original: 0x00b760a0 ped_task_zone_list_update (proposed)

/// Maintain a small list of task zones, then sweep the zone table.
///
/// The six float arguments are two triples, `lo = (a0, a1, a2)` and
/// `hi = (a3, a4, a5)`; `flags` (byte at the seventh word) selects the
/// mode. A global count guards a list of at most 12 six-float entries
/// held as six strided arrays.
///
/// When `flags` is zero the triples are appended as one entry if the list
/// holds fewer than 12. Otherwise the list is scanned for an entry whose
/// six floats all equal the arguments (numeric equality: signed zeros
/// match, NaN never does) and each match is removed by shifting the tail
/// down; the shift loop is verbatim from the original, including its two
/// quirks: a match in the last slot removes nothing at all, and any other
/// match shortens the list by about half of the remaining tail rather
/// than by one.
///
/// Afterwards the zone table (entries of 44 bytes) is swept: an entry
/// whose flag byte has bit 4 set and whose three anchor floats each lie
/// strictly between the matching `lo`/`hi` pair triggers one call, a
/// different callee for a zero or nonzero `flags`, and the nonzero case
/// also clears bit 0 of the entry's flag byte. Returns `flags & 0x20`.
///
/// Original: 0x00b760a0 (cdecl, seven stack words; the two callees take
/// no arguments).
unsafe fn run_00b760a0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, flags: u32, swap_calls: bool) -> u32 {
    unsafe {
        const COUNT_SLOT: u32 = 0x0167ca1c;
        const ARR_A: u32 = 0x0167cbe0;
        const ARR_B: u32 = 0x0167cb20;
        const ARR_C: u32 = 0x0167cbe4;
        const ARR_D: u32 = 0x0167cb24;
        const ARR_E: u32 = 0x0167cbe8;
        const ARR_F: u32 = 0x0167cb28;
        const LIST_CAP: i32 = 12;
        const SLOT_STRIDE: u32 = 0x10;
        const TABLE_BASE: u32 = 0x01670d08;
        const TABLE_END: u32 = 0x0167ca18;
        const ENTRY_STRIDE: u32 = 0x2c;
        const ENTRY_FLAG: u32 = 0x21;
        const FLAG_ARMED: u8 = 0x10;
        const ID_ON_FLAG: u32 = 1;
        const ID_ON_ZERO: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let f0 = f32::from_bits(a0);
        let f1 = f32::from_bits(a1);
        let f2 = f32::from_bits(a2);
        let f3 = f32::from_bits(a3);
        let f4 = f32::from_bits(a4);
        let f5 = f32::from_bits(a5);
        let flag = (flags & 0xff) as u8;
        let count_ptr = lf_checker_rt::relocated(COUNT_SLOT);
        let mut count = rd32(count_ptr) as i32;
        if flag == 0 {
            if count < LIST_CAP {
                let slot = (count as u32).wrapping_mul(SLOT_STRIDE);
                wr32(lf_checker_rt::relocated(ARR_A).wrapping_add(slot), a0);
                wr32(lf_checker_rt::relocated(ARR_B).wrapping_add(slot), a3);
                wr32(lf_checker_rt::relocated(ARR_C).wrapping_add(slot), a1);
                wr32(lf_checker_rt::relocated(ARR_D).wrapping_add(slot), a4);
                wr32(lf_checker_rt::relocated(ARR_E).wrapping_add(slot), a2);
                wr32(lf_checker_rt::relocated(ARR_F).wrapping_add(slot), a5);
                count += 1;
                wr32(count_ptr, count as u32);
            }
        } else if count > 0 {
            let mut ebp = 0i32;
            let mut edx = 0u32;
            while ebp < count {
                let hit = rdf(lf_checker_rt::relocated(ARR_A).wrapping_add(edx)) == f0
                    && rdf(lf_checker_rt::relocated(ARR_C).wrapping_add(edx)) == f1
                    && rdf(lf_checker_rt::relocated(ARR_E).wrapping_add(edx)) == f2
                    && rdf(lf_checker_rt::relocated(ARR_B).wrapping_add(edx)) == f3
                    && rdf(lf_checker_rt::relocated(ARR_D).wrapping_add(edx)) == f4
                    && rdf(lf_checker_rt::relocated(ARR_F).wrapping_add(edx)) == f5;
                if hit {
                    // Removal exactly as the original runs it: the two ends
                    // converge, so each iteration shifts one slot and drops
                    // the count by one until they meet.
                    let mut esi = count.wrapping_sub(1);
                    let mut ebx = ebp;
                    if ebp < esi {
                        let mut ecx = edx;
                        loop {
                            wr32(
                                lf_checker_rt::relocated(ARR_A).wrapping_add(ecx),
                                rd32(lf_checker_rt::relocated(ARR_A).wrapping_add(ecx).wrapping_add(SLOT_STRIDE)),
                            );
                            wr32(
                                lf_checker_rt::relocated(ARR_C).wrapping_add(ecx),
                                rd32(lf_checker_rt::relocated(ARR_C).wrapping_add(ecx).wrapping_add(SLOT_STRIDE)),
                            );
                            wr32(
                                lf_checker_rt::relocated(ARR_E).wrapping_add(ecx),
                                rd32(lf_checker_rt::relocated(ARR_E).wrapping_add(ecx).wrapping_add(SLOT_STRIDE)),
                            );
                            wr32(
                                lf_checker_rt::relocated(ARR_B).wrapping_add(ecx),
                                rd32(lf_checker_rt::relocated(ARR_B).wrapping_add(ecx).wrapping_add(SLOT_STRIDE)),
                            );
                            wr32(
                                lf_checker_rt::relocated(ARR_D).wrapping_add(ecx),
                                rd32(lf_checker_rt::relocated(ARR_D).wrapping_add(ecx).wrapping_add(SLOT_STRIDE)),
                            );
                            wr32(
                                lf_checker_rt::relocated(ARR_F).wrapping_add(ecx),
                                rd32(lf_checker_rt::relocated(ARR_F).wrapping_add(ecx).wrapping_add(SLOT_STRIDE)),
                            );
                            esi = esi.wrapping_sub(1);
                            ebx = ebx.wrapping_add(1);
                            count = count.wrapping_sub(1);
                            ecx = ecx.wrapping_add(SLOT_STRIDE);
                            if !(ebx < esi) {
                                break;
                            }
                        }
                        wr32(count_ptr, count as u32);
                    }
                }
                ebp += 1;
                edx = edx.wrapping_add(SLOT_STRIDE);
                count = rd32(count_ptr) as i32;
            }
        }
        let (id_nz, id_z) = if swap_calls { (ID_ON_ZERO, ID_ON_FLAG) } else { (ID_ON_FLAG, ID_ON_ZERO) };
        let mut entry = lf_checker_rt::relocated(TABLE_BASE);
        let end = lf_checker_rt::relocated(TABLE_END);
        while (entry as i32) < (end as i32) {
            let fb = (entry.wrapping_add(ENTRY_FLAG) as *const u8).read();
            if fb & FLAG_ARMED != 0 {
                let g0 = rdf(entry.wrapping_sub(8));
                if g0 > f0 && f3 > g0 {
                    let g1 = rdf(entry.wrapping_sub(4));
                    if g1 > f1 && f4 > g1 {
                        let g2 = rdf(entry);
                        if g2 > f2 && f5 > g2 {
                            if flag != 0 {
                                lf_checker_rt::callee_cdecl!(id_nz, u32,);
                                (entry.wrapping_add(ENTRY_FLAG) as *mut u8).write(fb & 0xfe);
                            } else {
                                lf_checker_rt::callee_cdecl!(id_z, u32,);
                            }
                        }
                    }
                }
            }
            entry = entry.wrapping_add(ENTRY_STRIDE);
        }
        (flag & 0x20) as u32
    }
}

lf_checker_rt::export!(cdecl, rw_00b760a0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe { run_00b760a0(a0, a1, a2, a3, a4, a5, a6, false) }
});
