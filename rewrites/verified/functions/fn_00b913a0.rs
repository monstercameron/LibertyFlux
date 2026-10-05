// original: 0x00b913a0 NativeImpl_SET_BLIP_COORDINATES

/// Moves a blip marker, but only for markers of kinds 4, 5 or 7.
///
/// Resolves `handle` through `LOOKUP`; a negative id returns at once.
/// Otherwise reads the marker row from `TABLE` (indexed by the id) and the
/// fallback row (indexed by the global `GLOB`): the row whose flag byte at
/// `FLAG_OFF` is set supplies the kind word at `KIND_OFF`, and the position
/// (`x`, `y`, `z`) packed into a frame buffer is applied through `APPLY`
/// with (2, `handle`, buffer) only when the kind is 4, 5 or 7. (The three
/// repeated kind checks in the original all read the same value.)
///
/// The buffer pointer is a skipped call argument with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B913A0 (cdecl, four stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b913a0(handle: u32, x: u32, y: u32, z: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const APPLY: u32 = 2;
        const TABLE: u32 = 0x0118F6F8;
        const GLOB: u32 = 0x01034494;
        const FLAG_OFF: u32 = 8;
        const KIND_OFF: u32 = 0x48;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let id: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, handle);
        if (id as i32) < 0 {
            return 0;
        }
        let base = lf_checker_rt::relocated(TABLE);
        let row = rd32(base.wrapping_add(id.wrapping_mul(4)));
        let g = lf_checker_rt::global::<u32>(GLOB).read();
        let krow = if (row.wrapping_add(FLAG_OFF) as *const u8).read() != 0 {
            row
        } else {
            rd32(base.wrapping_add(g.wrapping_mul(4)))
        };
        let kind = rd32(krow.wrapping_add(KIND_OFF));
        if kind != 4 && kind != 5 && kind != 7 {
            return 0;
        }
        let mut buf = [x, y, z];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            APPLY, u32, 2, handle, buf.as_mut_ptr() as u32
        );
        0
    }
});
