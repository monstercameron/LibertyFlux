// original: 0x00874020 crmt_forward_init3_zero_tag
// Forward to the three-word initializer with a zero tag followed by this
// call's two arguments. (thiscall/2)
export!(thiscall, rw_00874020(this_ptr: u32, arg0: u32, arg1: u32) -> () {
    callee_thiscall!(1, u32, this_ptr, 0, arg0, arg1);
});
