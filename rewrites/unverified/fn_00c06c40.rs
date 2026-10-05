// original: 0x00c06c40 stream_view_register_init
/// Register a fresh view object and initialise it from eight parts.
///
/// Prepares the table (thiscall/1 on `this`+8), allocates a 0x11c-byte view
/// (cdecl/1) default-initialised through the view helper (thiscall/0, or a
/// null slot when allocation fails), stores it at `arr`+`count`*4-4 (where
/// `arr` is `this`+8's pointer and `count` the word at `this`+0xc), forwards
/// all eight arguments to the parts initialiser (thiscall/8) on that slot,
/// and returns the slot. Thiscall: eight stack words, callee cleans 0x20.
lf_checker_rt::export!(thiscall, rw_00c06c40(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
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

        const PREP: u32 = 1;
        const ALLOC: u32 = 2;
        const VIEW: u32 = 3;
        const INIT: u32 = 4;
        const VIEW_SIZE: u32 = 0x11c;
        let _: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, this + 8, 0x10);
        let raw: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, VIEW_SIZE);
        let slot: u32 = if raw != 0 {
            lf_checker_rt::callee_thiscall!(VIEW, u32, raw)
        } else {
            0
        };
        let arr = rd32(this + 8);
        let count = rd16(this + 0x0c);
        wr32(arr.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4), slot);
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, slot, a0, a1, a2, a3, a4, a5, a6, a7);
        rd32(arr.wrapping_add(rd16(this + 0x0c).wrapping_mul(4)).wrapping_sub(4))
    }
});
