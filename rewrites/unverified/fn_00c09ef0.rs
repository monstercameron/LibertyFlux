// original: 0x00c09ef0 stream_flush_and_forward (proposed)

/// Flush the streaming queues, then tail-jump to the finisher.
///
/// `this` points to the queue set. The primer (callee 1, no arguments) runs
/// first. Each pair of the pair queue (`PAIRS`, 16-bit length at `NPAIR`,
/// 8 bytes per pair) goes to the pair helper (callee 2) with `this` in `ecx`.
/// Each row of the row queue (`ROWS`, 16-bit length at `NROW`, 32 bytes per
/// row) whose tag object (at `TAG` past the row, indirected through `TAGP`)
/// is set goes to the row helper (callee 3, five arguments: the row, the row
/// plus `R1`, the row's link plus 4 and plus `R2`, and the tag); a non-null
/// row answer receives the tag object at `ANSW`. Both lengths are cleared and
/// control tail-jumps to the finisher (callee 4) with `this` in `ecx`, whose
/// answer is returned.
///
/// Original: 0x00c09ef0 (thiscall, no stack words, ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_00c09ef0(this: u32) -> u32 {
    unsafe {
        const PAIRS: u32 = 0x522c;
        const NPAIR: u32 = 0x5230;
        const ROWS: u32 = 0x5224;
        const NROW: u32 = 0x5228;
        const ROW_STRIDE: u32 = 0x20;
        const TAG: u32 = 0x1c;
        const TAGP: u32 = 0x50;
        const R1: u32 = 0x10;
        const LINK: u32 = 0x0c;
        const R2: u32 = 0x38;
        const ANSW: u32 = 0x0c;
        const PRIME: u32 = 1;
        const PAIR: u32 = 2;
        const ROW: u32 = 3;
        const FINISH: u32 = 4;
        let _p: u32 = lf_checker_rt::callee_thiscall!(PRIME, u32, this);
        let np = (this.wrapping_add(NPAIR) as *const u16).read_unaligned() as u32;
        if np != 0 {
            let arr = (this.wrapping_add(PAIRS) as *const u32).read_unaligned();
            let mut i = 0u32;
            while (i as i32) < (np as i32) {
                let w0 = (arr.wrapping_add(i.wrapping_mul(8)) as *const u32).read_unaligned();
                let w1 = (arr.wrapping_add(i.wrapping_mul(8)).wrapping_add(4) as *const u32).read_unaligned();
                let _q: u32 = lf_checker_rt::callee_thiscall!(PAIR, u32, this, w0, w1);
                i += 1;
            }
        }
        let nr = (this.wrapping_add(NROW) as *const u16).read_unaligned() as u32;
        if nr != 0 {
            let rows = (this.wrapping_add(ROWS) as *const u32).read_unaligned();
            let mut j = 0u32;
            while (j as i32) < (nr as i32) {
                let row = rows.wrapping_add(j.wrapping_mul(ROW_STRIDE));
                let tag = (row.wrapping_add(TAG) as *const u32).read_unaligned();
                let tv = (tag.wrapping_add(TAGP) as *const u32).read_unaligned();
                if tv != 0 {
                    let link = (row.wrapping_add(LINK) as *const u32).read_unaligned();
                    let a: u32 = lf_checker_rt::callee_thiscall!(
                        ROW, u32, this, row, row.wrapping_add(R1),
                        link.wrapping_add(4), link.wrapping_add(R2), tv);
                    if a != 0 {
                        (a.wrapping_add(ANSW) as *mut u32).write_unaligned(tag);
                    }
                }
                j += 1;
            }
        }
        (this.wrapping_add(NROW) as *mut u16).write_unaligned(0);
        (this.wrapping_add(NPAIR) as *mut u16).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(FINISH, u32, this)
    }
});
