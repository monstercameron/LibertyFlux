// original: 0x00c06500 stream_set_field20
/// Store `value` into this streaming object's field at +0x20 when in range.
///
/// Values below 3 are stored; 3 and above (compared signed) leave the field
/// untouched. Returns nothing meaningful (EAX echoes the argument on the way
/// out, which the contract compares). Thiscall: `this` in ECX, one stack
/// word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c06500(this: u32, value: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x20;
        const LIMIT: i32 = 3;
        if (value as i32) < LIMIT {
            unsafe { ((this + FIELD) as *mut u32).write_unaligned(value) };
        }
        value
    }
});
