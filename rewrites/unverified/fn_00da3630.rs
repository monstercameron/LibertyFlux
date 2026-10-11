// original: 0x00DA3630 ped_attractor_apply_item_if_valid (proposed)

/// Apply an item only when the secondary pointer is non-null and both member
/// validation helpers accept it. The first helper receives the primary
/// argument with the current attractor as `this`; the second receives the
/// secondary pointer. On success, the primary argument is forwarded to the
/// secondary object's registration method and that method's low-byte result
/// is returned. Failure at any gate returns low byte zero.
///
/// The method uses the merged attractor pointer in ECX, two 32-bit stack
/// arguments, thiscall cleanup of eight bytes, and no direct memory fields.
lf_checker_rt::export!(thiscall, rw_00da3630(self_ptr: u32, primary: u32, secondary: u32) -> u8 {
    if secondary == 0 {
        return 0;
    }
    let eligible = lf_checker_rt::callee_thiscall!(1, u32, self_ptr, primary);
    if eligible as u8 == 0 {
        return 0;
    }
    let available = lf_checker_rt::callee_thiscall!(2, u32, self_ptr, secondary);
    if available as u8 == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(3, u32, secondary, primary) as u8
});
