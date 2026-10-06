// original: 0x0065B040 shader_pass_copy (proposed)

/// Copy one shader pass descriptor, stamping the class vtable pointer.
///
/// `this` is the destination, the stack argument the source. Writes the
/// vtable pointer at `+0x00`, copies 16 dwords from `+4` (`rep movsd`),
/// five dwords at `+0x44`..`+0x54`, one byte at `+0x58`, then six 8-byte
/// blocks at `+0x5c`..`+0x84` (the original moves them through a vector
/// register; the rewrite copies them as integers, bit-identical). Bytes
/// `+0x59`..`+0x5b` are not copied. Returns `this` (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0065b040(this: u32, src: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xFE2E48;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let mut i = 0u32;
        while i < 16 {
            wr32(this + 4 + i * 4, rd32(src + 4 + i * 4));
            i += 1;
        }
        wr32(this + 0x44, rd32(src + 0x44));
        wr32(this + 0x48, rd32(src + 0x48));
        wr32(this + 0x4c, rd32(src + 0x4c));
        wr32(this + 0x50, rd32(src + 0x50));
        wr32(this + 0x54, rd32(src + 0x54));
        ((this + 0x58) as *mut u8).write_unaligned(((src + 0x58) as *const u8).read_unaligned());
        wr64(this + 0x5c, rd64(src + 0x5c));
        wr64(this + 0x64, rd64(src + 0x64));
        wr64(this + 0x6c, rd64(src + 0x6c));
        wr64(this + 0x74, rd64(src + 0x74));
        wr64(this + 0x7c, rd64(src + 0x7c));
        wr64(this + 0x84, rd64(src + 0x84));
        this
    }
});
