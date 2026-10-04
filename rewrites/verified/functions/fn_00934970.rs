// original: 0x00934970 append_slot
/// 0x00934970: append a 0x22c-stride slot. When the array is full, grow it
/// by the requested amount through the array allocator, copy the live
/// records across, release the old block, and then take the new slot.
/// Returns a pointer to the fresh slot.
// shared layout constant
const HDR_CAP: usize = 6;
// shared layout constant
const HDR_DATA: usize = 0;
// shared layout constant
const HDR_LEN: usize = 4;
// shared layout constant
const SCAN_STRIDE: u32 = 0x22c;
export!(thiscall, rw_00934970(this: *mut u8, grow: u32) -> u32 {
    unsafe {
        let len = core::ptr::read_unaligned(this.add(HDR_LEN) as *const u16);
        let cap = core::ptr::read_unaligned(this.add(HDR_CAP) as *const u16);
        if len != cap {
            let base = core::ptr::read_unaligned(this.add(HDR_DATA) as *const u32);
            let slot = base.wrapping_add((len as u32).wrapping_mul(SCAN_STRIDE));
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
                    oldbase.wrapping_add((i as u32).wrapping_mul(SCAN_STRIDE));
                let dst =
                    newbase.wrapping_add((i as u32).wrapping_mul(SCAN_STRIDE));
                core::ptr::copy_nonoverlapping(
                    src as *const u8,
                    dst as *mut u8,
                    SCAN_STRIDE as usize,
                );
            }
            let old = core::ptr::read_unaligned(this.add(HDR_DATA) as *const u32);
            callee_cdecl!(2, u32, old);
            core::ptr::write_unaligned(this.add(HDR_DATA) as *mut u32, newbase);
            let len2 = core::ptr::read_unaligned(this.add(HDR_LEN) as *const u16);
            let slot = newbase.wrapping_add((len2 as u32).wrapping_mul(SCAN_STRIDE));
            core::ptr::write_unaligned(
                this.add(HDR_LEN) as *mut u16,
                len2.wrapping_add(1),
            );
            slot
        }
    }
});
