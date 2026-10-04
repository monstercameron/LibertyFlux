// original: 0x00e635e0 dyninit_array2_zero_g11a1e48
/// Construct two strided global objects and zero each element window.
/// The last constructor answer passes through in EAX, as in the original.
export!(cdecl, rw_00e635e0() -> u32 {
    let mut ans = 0u32;
    for i in 0..2u32 {
        let base = relocated(0x11A1E48).wrapping_add(i.wrapping_mul(0x270));
        callee_thiscall!(0, u32, base.wrapping_sub(0x198));
        callee_stdcall!(1, u32, base.wrapping_sub(0x158), 0x40, 4, relocated(0x4065E0));
        ans = callee_thiscall!(0, u32, base.wrapping_sub(0x58));
        unsafe {
            let w = (base.wrapping_sub(8)) as *mut u32;
            for k in 0..16usize {
                *w.add(k) = 0;
            }
        }
    }
    ans
});
