// original: 0x00951C50 files_collect_flagged_entries (proposed)

/// Collect flagged entries from three global object tables, then process each one.
///
/// Each of the three tables is described by a four-word header kept in the
/// game's data: the entry array base, a parallel per-index flag-bytes array,
/// a signed entry count and the byte stride between entries. The scan visits
/// every index below the count (which is compared as a signed value: a zero
/// or negative count selects nothing) and keeps an entry only when its flag
/// byte lacks bit 0x80, the computed entry address is non-null, and the
/// entry's status word at offset 0x24 has bit 0x400 set. The third table
/// additionally requires bit 0x20000 at offset 0x210 to be clear. At most
/// 256 entries are kept. Every kept entry is handed, through a pointer to
/// its own collection slot, to the registration callee (id 0).
///
/// Afterwards each collected slot is re-read (the callees may have rewritten
/// it) and processed in order: null slots are skipped; when the low byte of
/// the `flag` argument is non-zero the entry is first unlinked through callee
/// id 2 with the key kept at entry offset 0x64; non-null slots go through the
/// detach callee (id 1, again via a slot pointer) and then the release callee
/// (id 3, by value). Finally the rundown callee (id 4) runs against a fixed
/// global object and its answer is returned.
///
/// Original: 0x00951C50 (cdecl, one stack word of which only the low byte is
/// read; no register inputs; returns the rundown callee's answer).
lf_checker_rt::export!(cdecl, rw_00951C50(flag: u32) -> u32 {
    unsafe {
        const TABLE_COUNT: usize = 3;
        /// File addresses of the three table headers (base, flag bytes, signed count, stride).
        const HEADERS: [u32; TABLE_COUNT] = [0x012E22A4, 0x018B6F1C, 0x01632C60];
        const HDR_BASE: usize = 0;
        const HDR_FLAGS: usize = 1;
        const HDR_COUNT: usize = 2;
        const HDR_STRIDE: usize = 3;
        const FLAG_SKIP: u8 = 0x80;
        const ENTRY_STATUS: u32 = 0x24;
        const STATUS_KEPT: u32 = 0x400;
        const ENTRY_EXTRA: u32 = 0x210;
        const EXTRA_REJECT: u32 = 0x20000;
        const ENTRY_KEY: u32 = 0x64;
        const MAX_KEPT: usize = 0x100;
        const CALLEE_REGISTER: u32 = 0;
        const CALLEE_DETACH: u32 = 1;
        const CALLEE_UNLINK: u32 = 2;
        const CALLEE_RELEASE: u32 = 3;
        const CALLEE_RUNDOWN: u32 = 4;
        const RUNDOWN_OBJECT: u32 = 0x01683290;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn header(file_va: u32) -> [u32; 4] {
            unsafe { *lf_checker_rt::global::<[u32; 4]>(file_va) }
        }

        let mut slots = [0u32; MAX_KEPT];
        let mut kept = 0usize;
        for t in 0..TABLE_COUNT {
            let h = header(HEADERS[t]);
            let (base, flags, stride) = (h[HDR_BASE], h[HDR_FLAGS], h[HDR_STRIDE]);
            let count = h[HDR_COUNT] as i32;
            if count <= 0 {
                continue;
            }
            let mut i = 0i32;
            while i < count {
                let index = i as u32;
                if rd8(flags.wrapping_add(index)) & FLAG_SKIP == 0 {
                    let entry = base.wrapping_add(stride.wrapping_mul(index));
                    // Order matches the original: null, status bit, third-table
                    // extra bit, then capacity. Each read happens only if the
                    // earlier tests passed.
                    let mut keep = entry != 0
                        && rd32(entry.wrapping_add(ENTRY_STATUS)) & STATUS_KEPT != 0;
                    if keep && t == 2 {
                        keep = rd32(entry.wrapping_add(ENTRY_EXTRA)) & EXTRA_REJECT == 0;
                    }
                    if keep && kept < MAX_KEPT {
                        slots[kept] = entry;
                        let slot = core::ptr::addr_of_mut!(slots[kept]) as u32;
                        lf_checker_rt::callee_thiscall!(CALLEE_REGISTER, u32, entry, slot);
                        kept += 1;
                    }
                }
                i += 1;
            }
        }

        if kept > 0 {
            let unlink = (flag & 0xFF) != 0;
            for i in 0..kept {
                // Re-read before every step: the callees write back through the slot.
                let first = slots[i];
                if first == 0 {
                    continue;
                }
                if unlink {
                    let key = rd32(first.wrapping_add(ENTRY_KEY));
                    lf_checker_rt::callee_cdecl!(CALLEE_UNLINK, u32, 1u32, key, first);
                }
                let second = slots[i];
                if second != 0 {
                    let slot = core::ptr::addr_of_mut!(slots[i]) as u32;
                    lf_checker_rt::callee_thiscall!(CALLEE_DETACH, u32, second, slot);
                }
                let third = slots[i];
                lf_checker_rt::callee_cdecl!(CALLEE_RELEASE, u32, third);
            }
        }

        lf_checker_rt::callee_thiscall!(
            CALLEE_RUNDOWN,
            u32,
            lf_checker_rt::relocated(RUNDOWN_OBJECT)
        )
    }
});
