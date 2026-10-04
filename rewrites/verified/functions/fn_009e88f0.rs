// original: 0x009e88f0 ped_max_of_pair
/// Greater of the floats at `+0xE0` and `+0xE4`, returned on the x87
/// stack. The comparison is ordered-greater only, so NaN, equality and
/// less-than all yield the second slot. (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e88f0(this_ptr: u32) -> f32 {
    unsafe {
        const A_OFF: u32 = 0xE0;
        const B_OFF: u32 = 0xE4;
        let a = f32::from_bits((this_ptr.wrapping_add(A_OFF) as *const u32).read_unaligned());
        let b = f32::from_bits((this_ptr.wrapping_add(B_OFF) as *const u32).read_unaligned());
        if a > b { a } else { b }
    }
});
