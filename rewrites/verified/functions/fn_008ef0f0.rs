// original: 0x008ef0f0 remove_entry_and_requery
use lf_checker_rt::{callee_thiscall, export};

/// Drop one binding entry and re-run the range query (original 0x008EF0F0).
///
/// Scans the entry table for records whose flag byte is set and whose key
/// matches, removing each by shifting the tail down. When at least one
/// record was removed, re-issues the spatial query for all 64 slots over
/// the full coordinate range and then once per surviving record over that
/// record's own box. Returns nothing meaningful.
export!(thiscall, rw_008ef0f0(this: u32, key: u32) -> u32 {
    const COUNT_OFF: usize = 0xE08;
    const RECS_OFF: usize = 0xE0C;
    const REC_LEN: usize = 0x20;
    const KEY_OFF: usize = 0x18;
    const FLAG_OFF: usize = 0x1D;
    const TAG_OFF: usize = 0x1C;
    const POS_RANGE: u32 = 0x461C4000; // +10000.0
    const NEG_RANGE: u32 = 0xC61C4000; // -10000.0
    const SLOTS: u32 = 0x40;
    unsafe {
        let base = this as *mut u8;
        let countp = base.add(COUNT_OFF) as *mut u32;
        if (countp.read() as i32) <= 0 {
            return 0;
        }
        let mut removed = false;
        let mut idx: i32 = 0;
        loop {
            let n = countp.read() as i32;
            if idx >= n {
                break;
            }
            let rec = base.add(RECS_OFF + (idx as usize) * REC_LEN);
            let flag = rec.add(FLAG_OFF).read();
            let kval = (rec.add(KEY_OFF) as *const u32).read();
            if flag != 0 && kval == key {
                removed = true;
                let mut j = idx;
                while j + 1 < n {
                    let dst = base.add(RECS_OFF + (j as usize) * REC_LEN);
                    core::ptr::copy_nonoverlapping(dst.add(REC_LEN), dst, REC_LEN);
                    j += 1;
                }
                countp.write((n - 1) as u32);
            } else {
                idx += 1;
            }
        }
        if !removed {
            return 0;
        }
        let mut grp: u32 = 0;
        while grp < SLOTS {
            callee_thiscall!(1, u32, this,
                NEG_RANGE, POS_RANGE, NEG_RANGE, POS_RANGE, NEG_RANGE, POS_RANGE,
                0, grp, 1);
            let n = countp.read() as i32;
            let mut r: i32 = 0;
            while r < n {
                let rec = base.add(RECS_OFF + (r as usize) * REC_LEN) as *const u32;
                let f0 = rec.read();
                let f1 = rec.add(1).read();
                let f2 = rec.add(2).read();
                let f3 = rec.add(3).read();
                let f4 = rec.add(4).read();
                let f5 = rec.add(5).read();
                let tag = (rec as *const u8).add(TAG_OFF).read() as u32;
                callee_thiscall!(1, u32, this, f0, f1, f2, f3, f4, f5, tag, grp, 0);
                r += 1;
            }
            grp = grp.wrapping_add(1);
        }
    }
    0
});
