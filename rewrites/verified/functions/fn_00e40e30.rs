// original: 0x00e40e30 row_slot_init_copy
// row slot init plus 16-byte header copy.
// Runs the per-row initializer on base+index*0x2b0, then copies 16 bytes
// from src to the slot start. Returns src.
export!(stdcall, rw_00e40e30(base: u32, index: u32, src: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        let elem = base.wrapping_add(index.wrapping_mul(STRIDE));
        callee_thiscall!(1, u32, elem);
        let dst = elem as *mut u64;
        let from = src as *const u64;
        core::ptr::write_unaligned(dst, core::ptr::read_unaligned(from));
        core::ptr::write_unaligned(
            dst.add(1),
            core::ptr::read_unaligned(from.add(1)),
        );
        src
    }
});
