// original: 0x008DDE30 CSetGeometryVertexOffsets::vf1

/// CSetGeometryVertexOffsets::vf1 (draw-command virtual slot 1).
///
/// Sets the geometry vertex offsets through the geometry object held
/// at +0x08: the slot at its vtable +0x1c is called with that object
/// as `this` and the offsets word at +0x0c.
///
/// Original: 0x008DDE30 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dde30(this: u32) -> u32 {
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
        const GEOMETRY: u32 = 0x08;
        const OFFSETS: u32 = 0x0c;
        const SET_SLOT: u32 = 0x1c;
        let geo = rd32(this + GEOMETRY);
        let vtable = (geo as *const u32).read();
        let target = (vtable.wrapping_add(SET_SLOT) as *const u32).read();
        let set: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        set(geo, rd32(this + OFFSETS));
        0
    }
});
