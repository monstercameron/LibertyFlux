// original: 0x00934ba0 slot_has_mark
/// 0x00934BA0: report whether any record matching the target id (gated on
/// +0x226, id byte +0x229) also carries the secondary mark (+0x225). A
/// zero target with a clear state flag matches trivially. Only AL defined.
// shared layout constant
const SCAN_STRIDE: u32 = 0x22c;
// shared layout constant
const SET_COUNT: usize = 0x148;
// shared layout constant
const SET_RECORDS: usize = 0x144;
export!(thiscall, rw_00934ba0(this: *const u8, target: u32) -> u8 {
    if target == 0 && unsafe { core::ptr::read((this as *const u8).add(0x169)) } == 0 {
        return 1;
    }
    callee_thiscall!(1, u32, this as u32);
    callee_thiscall!(2, u32, this as u32);
    let count =
        unsafe { core::ptr::read_unaligned(this.add(SET_COUNT) as *const u16) } as i32;
    let mut hit = false;
    if count > 0 {
        let base =
            unsafe { core::ptr::read_unaligned(this.add(SET_RECORDS) as *const u32) };
        for i in 0..count {
            let rec = base.wrapping_add((i as u32).wrapping_mul(SCAN_STRIDE));
            unsafe {
                if core::ptr::read(rec.wrapping_add(0x226) as *const u8) == 0 {
                    continue;
                }
                let v = core::ptr::read(rec.wrapping_add(0x229) as *const u8) as u32;
                if v != target {
                    continue;
                }
                if core::ptr::read(rec.wrapping_add(0x225) as *const u8) != 0 {
                    hit = true;
                }
            }
        }
    }
    hit as u8
});
