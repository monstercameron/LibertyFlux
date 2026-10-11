// original: 0x00cf9e40 CTaskSimpleClimbLadder::vf5
/// Checks a count returned by the query object's vtable as a signed 32-bit
/// value against 0x58. For smaller counts it queries a float from the first
/// object's vtable and rejects positive scores. It then rejects the sentinel
/// query kind 0x42; otherwise it forwards the first object to a helper and returns
/// a bool in AL. A null query object skips both queries and calls the helper.
lf_checker_rt::export!(thiscall, rw_00cf9e40(this: *mut u8, first_object: *mut u8, argument_two: u32, query_object: *mut u8) -> u32 {
    unsafe {
        const REJECT_KIND: u32 = 0x42;
        if !query_object.is_null() {
            if argument_two != 2 {
                let count = lf_checker_rt::callee_thiscall!(1, u32, query_object as u32);
                if (count as i32) < 0x58 {
                    let score: f32 = lf_checker_rt::callee_thiscall!(2, f32, first_object as u32);
                    if score > 0.0 {
                        return 0;
                    }
                }
            }
            if lf_checker_rt::callee_thiscall!(3, u32, query_object as u32) == REJECT_KIND {
                return 0;
            }
        }
        lf_checker_rt::callee_thiscall!(4, u32, this as u32, first_object as u32);
        1
    }
});
