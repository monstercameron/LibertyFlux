// original: 0x00c3ac20 CTrain::vf59
/// Copy the three-word field at `+0x1450` of this train car into `dst`.
///
/// `this` (ECX) points to the car, `dst` is the caller's three-word buffer.
/// Reads dwords at `+0x1450`, `+0x1454`, `+0x1458` and stores them at
/// `dst[0..3]`. Returns `dst` unchanged (EAX still holds the argument).
/// No branches, no calls.
///
/// Original: 0x00c3ac20 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3ac20(this: u32, dst: u32) -> u32 {
    unsafe {
        const FIELD0: u32 = 0x1450;
        const FIELD1: u32 = 0x1454;
        const FIELD2: u32 = 0x1458;
        ((dst as *mut u32)).write_unaligned(((this + FIELD0) as *const u32).read_unaligned());
        ((dst + 4) as *mut u32).write_unaligned(((this + FIELD1) as *const u32).read_unaligned());
        ((dst + 8) as *mut u32).write_unaligned(((this + FIELD2) as *const u32).read_unaligned());
        dst
    }
});
