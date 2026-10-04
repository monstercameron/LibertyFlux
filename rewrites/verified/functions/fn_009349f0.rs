// original: 0x009349f0 append_detail
/// 0x009349F0: append a 0x168-stride detail record. Same grow/copy/release
/// shape as 0x00934970, plus a release pass that drops each live record's
/// attached object (at +0x154) through its first virtual slot before the
/// old block is freed. Returns a pointer to the fresh record.
// shared layout constant
const HDR_CAP: usize = 6;
// shared layout constant
const HDR_DATA: usize = 0;
// shared layout constant
const HDR_LEN: usize = 4;
// shared layout constant
const REC_B_STRIDE: u32 = 0x168;
export!(thiscall, rw_009349f0(this: *mut u8, grow: u32) -> u32 {
    unsafe {
        let len = core::ptr::read_unaligned(this.add(HDR_LEN) as *const u16);
        let cap = core::ptr::read_unaligned(this.add(HDR_CAP) as *const u16);
        if len != cap {
            let base = core::ptr::read_unaligned(this.add(HDR_DATA) as *const u32);
            let slot = base.wrapping_add((len as u32).wrapping_mul(REC_B_STRIDE));
            core::ptr::write_unaligned(
                this.add(HDR_LEN) as *mut u16,
                len.wrapping_add(1),
            );
            slot
        } else {
            let newcap = cap.wrapping_add(grow as u16);
            core::ptr::write_unaligned(this.add(HDR_CAP) as *mut u16, newcap);
            let newbase = callee_stdcall!(1, u32, newcap as u32);
            let oldbase = core::ptr::read_unaligned(this.add(HDR_DATA) as *const u32);
            // The copy guard compares a zeroed register against the count,
            // so the copy runs whenever any record is live.
            for i in 0..len {
                let src =
                    oldbase.wrapping_add((i as u32).wrapping_mul(REC_B_STRIDE));
                let dst =
                    newbase.wrapping_add((i as u32).wrapping_mul(REC_B_STRIDE));
                core::ptr::copy_nonoverlapping(
                    src as *const u8,
                    dst as *mut u8,
                    REC_B_STRIDE as usize,
                );
            }
            let live = core::ptr::read_unaligned(this.add(HDR_LEN) as *const u16);
            for i in 0..live {
                let rec =
                    oldbase.wrapping_add((i as u32).wrapping_mul(REC_B_STRIDE));
                let obj = core::ptr::read_unaligned(
                    rec.wrapping_add(0x154) as *const u32,
                );
                if obj != 0 {
                    let vt = core::ptr::read_unaligned(obj as *const u32);
                    let slot0 = core::ptr::read_unaligned(vt as *const u32);
                    let release: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(slot0 as usize);
                    release(obj, 1);
                }
            }
            let old = core::ptr::read_unaligned(this.add(HDR_DATA) as *const u32);
            callee_cdecl!(2, u32, old);
            core::ptr::write_unaligned(this.add(HDR_DATA) as *mut u32, newbase);
            let len2 = core::ptr::read_unaligned(this.add(HDR_LEN) as *const u16);
            let slot = newbase.wrapping_add((len2 as u32).wrapping_mul(REC_B_STRIDE));
            core::ptr::write_unaligned(
                this.add(HDR_LEN) as *mut u16,
                len2.wrapping_add(1),
            );
            slot
        }
    }
});
