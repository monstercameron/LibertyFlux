// original: 0x00addc10 CRenderPhasePreRenderViewport::vf8

/// Build the pre-render viewport's device context and align its
/// channel field to a 16-byte boundary.
///
/// `this` is the phase object. The allocator callee (id 1: size
/// 0x20, flags 0) provides a builder; when it returns null the
/// function faults on the null object. Otherwise the construct callee
/// (id 2: builder, the word at `this+0x940`, then 0xb01ab0, 0, 0, 0)
/// builds the context (the 0xb01ab0 word is an image-pointer
/// immediate, relocated with the image), and the context's table slot
/// 2 (id 3, through the returned object's table) is called twice: the
/// results
/// `r1` and `r2` feed `(r2 + (16 - r1 mod 16) mod 16) / 16`, whose
/// value is deposited into bits 13..22 of the word at context `+0x4`.
///
/// Edge cases: a null builder faults identically on both sides; the
/// divisions are truncating, exact for negative results too.
///
/// Original: thiscall, no stack arguments. Callee id 1 is cdecl with
/// two arguments, id 2 thiscall with five, id 3 thiscall with none
/// through the fabricated table. Returns nothing.
lf_checker_rt::export!(thiscall, rw_00addc10(this: u32) -> u32 {
    unsafe {
        const PARAM: u32 = 0x940;
        const MAGIC: u32 = 0x00B01AB0;
        const FIELD: u32 = 0x4;
        const FIELD_MASK: u32 = 0x01FFC000;
        let builder: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0x20, 0);
        // A null builder skips construction and falls into the table
        // call on the null object; fault on the same read as the original.
        if builder == 0 {
            core::ptr::read_volatile(0 as *const u32);
        }
        // MAGIC is an image pointer immediate, relocated with the image.
        let magic = lf_checker_rt::relocated(MAGIC);
        let ctx: u32 = lf_checker_rt::callee_thiscall!(
            2, u32, builder,
            ((this + PARAM) as *const u32).read_unaligned(), magic, 0, 0, 0
        );
        let table = (ctx as *const u32).read_unaligned();
        let slot = ((table + 8) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let r1 = f(ctx) as i32;
        let e = (16 - r1 % 16) % 16;
        let r2 = f(ctx) as i32;
        let q = r2.wrapping_add(e) / 16;
        let cell = ((ctx + FIELD) as *const u32).read_unaligned();
        let dep = ((q as u32).wrapping_shl(14) ^ cell) & FIELD_MASK;
        ((ctx + FIELD) as *mut u32).write_unaligned(cell ^ dep);
    }
    0
});
