// original: 0x00b01710 apply_float_quad
/// Fetch a float block through the sub-object, build a local parameter
/// quad [f0, f1, f2+f0, f3+f1], and run the applier over it twice (whole
/// quad, then its upper half) with the incoming arguments. Later block
/// words are loaded but never used. Returns the second result.
export!(thiscall, rw_00b01710(this_: *mut u8, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    let blk: u32 = callee_thiscall!(1, u32, unsafe { this_.add(0x10) } as u32);
    let at = |i: usize| unsafe { ((blk as *const f32).add(i)).read_unaligned() };
    let (f0, f1, f2, f3) = (at(0), at(1), at(2), at(3));
    let mut local = [f0, f1, f2 + f0, f3 + f1];
    let _: u32 = callee_thiscall!(
        2,
        u32,
        unsafe { this_.add(0x46c) } as u32,
        local.as_mut_ptr() as u32,
        a0,
        a2,
        a3
    );
    callee_thiscall!(
        3,
        u32,
        unsafe { this_.add(0x48c) } as u32,
        unsafe { local.as_mut_ptr().add(2) } as u32,
        a1,
        a2,
        a3
    )
});
