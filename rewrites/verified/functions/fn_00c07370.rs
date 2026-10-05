// original: 0x00c07370 stream_slot_remove_at
/// Remove the slot at `index`, compacting the rest down.
///
/// Does nothing when `index` is at or past the slot count at +4 (signed).
/// Otherwise initialises the doomed slot through the slot helper (thiscall/0),
/// unregisters its +0x28 extension through the unregister helper unless that
/// address is null (thiscall/2), and compacts the array through the compact
/// helper (thiscall/1). Returns the compact helper's answer on the removal
/// path, the count otherwise. Thiscall: one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c07370(this: u32, index: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const SLOT_FN: u32 = 1;
        const UNREG: u32 = 2;
        const COMPACT: u32 = 3;
        const COUNT_OFF: u32 = 4;
        const SLOT_STRIDE: u32 = 80;
        const EXT_OFF: u32 = 0x28;
        let count = rd16(this + COUNT_OFF) as i32;
        if (index as i32) >= count {
            return count as u32;
        }
        let slot = rd32(this).wrapping_add((index as u32).wrapping_mul(SLOT_STRIDE));
        let _: u32 = lf_checker_rt::callee_thiscall!(SLOT_FN, u32, slot);
        let ext = slot.wrapping_add(EXT_OFF);
        if ext != 0 {
            let n = rd16(ext + 4);
            let arr = rd32(ext);
            let _: u32 = lf_checker_rt::callee_thiscall!(UNREG, u32, ext, arr, arr.wrapping_add(n.wrapping_mul(4)));
        }
        lf_checker_rt::callee_thiscall!(COMPACT, u32, this, index)
    }
});
