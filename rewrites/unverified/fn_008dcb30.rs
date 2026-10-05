// original: 0x008DCB30 CCreateRenderListGroupDC::vf1

/// CCreateRenderListGroupDC::vf1 (draw-command virtual slot 1).
///
/// Allocates a 768 KiB render-list block from the global allocator
/// object, then creates the render-list group on the global render-list
/// manager with the four words at +0x08..+0x14. Both callees pop their
/// own arguments (thiscall); the object pointers are absolute globals.
///
/// Original: 0x008DCB30 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dcb30(this: u32) -> u32 {
    unsafe {
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
        const ALLOCATOR: u32 = 0x01175C58;
        const MANAGER: u32 = 0x01593318;
        const BLOCK_BYTES: u32 = 0xc0000;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(ALLOCATOR), BLOCK_BYTES, 0xffffffffu32);
        lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(MANAGER), rd32(this + 8),
            rd32(this + 0xc), rd32(this + 0x10), rd32(this + 0x14));
        0
    }
});
