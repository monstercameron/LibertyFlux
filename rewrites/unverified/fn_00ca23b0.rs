// original: 0x00ca23b0 ped_flag_check

/// Decide a ped event from the counter helper and two stored flags.
///
/// Queries the counter (callees 1 and 2 in sequence). When the count is
/// positive and the kind word at `+0x12c` is 2, the answer is the count with
/// a zero low byte. Otherwise a non-zero flag at `+0x143` answers the count
/// with low byte 1. Otherwise the stack word and a global go to the
/// fallback helper (callee 3) whose answer is returned as is.
///
/// Original: 0x00ca23b0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ca23b0(this: u32, arg: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe {
        const KIND: u32 = 0x12c;
        const FLAG: u32 = 0x143;
        const KIND_PED: u32 = 2;
        let h: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        let n: u32 = lf_checker_rt::callee_thiscall!(2, u32, h);
        if (n as i32) > 0 && rd32(this + KIND) == KIND_PED {
            return n & 0xffff_ff00;
        }
        if rd8(this + FLAG) != 0 {
            return (n & 0xffff_ff00) | 1;
        }
        lf_checker_rt::callee_cdecl!(3, u32, arg, *lf_checker_rt::global::<u32>(0x012b_4138))
    }
});
