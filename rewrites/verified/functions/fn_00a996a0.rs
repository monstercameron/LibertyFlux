// original: 0x00a996a0 filemem_device_matches

/// Report whether the global device answers `arg` with a wanted kind.
///
/// Asks the object behind the global at `0x018b8968` (through its vtable
/// slot at `+0x14`, a thiscall taking `arg`) for a descriptor, and returns
/// 1 when the descriptor's kind word at `+0x20` is 0x1e. Otherwise asks
/// once more and returns 1 when that descriptor's kind is 0x1f, else 0.
/// Both kind comparisons are 16-bit equality checks.
///
/// Original: 0x00A996A0 (stdcall, one stack word; one indirect callee).
lf_checker_rt::export!(stdcall, rw_00a996a0(arg: u32) -> u32 {
    unsafe {
        /// Global device object pointer (file VA).
        const DEVICE: u32 = 0x018b8968;
        /// Query slot in the device vtable.
        const VT_QUERY: u32 = 0x14;
        /// Kind word, from a descriptor.
        const KIND_OFF: u32 = 0x20;
        /// Wanted kinds, first and second try.
        const KIND_A: u16 = 0x1e;
        const KIND_B: u16 = 0x1f;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let obj = rd32(lf_checker_rt::relocated(DEVICE));
        let slot = rd32(rd32(obj).wrapping_add(VT_QUERY));
        let query: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let r1 = query(obj, arg);
        if ((r1.wrapping_add(KIND_OFF)) as *const u16).read_unaligned() == KIND_A {
            return 1;
        }
        let r2 = query(obj, arg);
        if ((r2.wrapping_add(KIND_OFF)) as *const u16).read_unaligned() == KIND_B {
            return 1;
        }
        0
    }
});
