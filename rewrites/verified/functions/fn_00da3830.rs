// original: 0x00DA3830 CPedShelterAttractor::vf5

/// Validate the single item through the attractor's signed status helper.
/// A negative status returns low byte zero. A nonnegative status removes the
/// item from the three member collections and returns low byte one. The
/// status is signed; values with bit 31 set take the rejection path.
///
/// `self_ptr` is in ECX and `item` is the one 32-bit stack argument. The
/// helpers are called with `self_ptr` as their thiscall receiver.
lf_checker_rt::export!(thiscall, rw_00da3830(self_ptr: u32, item: u32) -> u8 {
    let status = lf_checker_rt::callee_thiscall!(1, u32, self_ptr, item);
    if (status as i32) < 0 {
        return 0;
    }
    let _ = lf_checker_rt::callee_thiscall!(2, u32, self_ptr, item);
    let _ = lf_checker_rt::callee_thiscall!(3, u32, self_ptr, item);
    let _ = lf_checker_rt::callee_thiscall!(4, u32, self_ptr);
    1
});
