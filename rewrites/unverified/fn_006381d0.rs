// original: 0x006381D0 rage::ptxEffectRuleStd::update_child_rules

/// Visit each child pointer in the array at `this + 0x130`, using the
/// zero-extended 16-bit count at `this + 0x134`. Each child receives one
/// no-argument call through vtable slot `0x28`, with the child in ECX. The
/// original's initial bounds check is unsigned and its loop branch is signed;
/// for this zero-extended 16-bit count, the loop still stops at the count.
/// Return the child count in EAX. Inputs above 257 children are not covered by
/// this proof because the contract keeps every child call inside its log cap.
lf_checker_rt::export!(thiscall, rw_006381D0(this: u32) -> u32 {
    const CHILD_POINTERS: u32 = 0x130;
    const CHILD_COUNT: u32 = 0x134;
    const UPDATE_CHILD: u32 = 1;

    unsafe {
        let count = (this.wrapping_add(CHILD_COUNT) as *const u16).read_unaligned() as u32;
        if count != 0 {
            let children = (this.wrapping_add(CHILD_POINTERS) as *const u32).read_unaligned();
            let mut index = 0u32;
            while index < count {
                let child = (children.wrapping_add(index.wrapping_mul(4)) as *const u32)
                    .read_unaligned();
                let _ = lf_checker_rt::callee_thiscall!(UPDATE_CHILD, u32, child);
                index = index.wrapping_add(1);
            }
        }
        count
    }
});
