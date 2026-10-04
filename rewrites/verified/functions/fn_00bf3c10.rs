// original: 0x00bf3c10 copy_triple
/// Resolve through the helper, then copy words +8/+0xC/+0x10 to `out`.
export!(thiscall, rw_bf3c10(this: *const u8, a0: u32, out: *mut u32) -> u32 {
    unsafe {
        let t4 = *((this.add(4)) as *const u32);
        let _: u32 = callee_cdecl!(1, u32, a0, t4);
        let w0 = *((this.add(8)) as *const u32);
        let w1 = *((this.add(0xC)) as *const u32);
        let w2 = *((this.add(0x10)) as *const u32);
        *out.add(0) = w0;
        *out.add(1) = w1;
        *out.add(2) = w2;
        w2
    }
});
