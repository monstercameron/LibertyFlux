// original: 0x008a60b0 aud_math_op_notify_and_forward
/// Notify a listener, then forward two arguments to a table-selected worker.
///
/// Resolves a target pointer from the audio entity table for the index/count
/// bytes (+0x40, +0x48), returning 1 when the count is 0xff or the total is
/// 0. Otherwise notifies the target with the word at +0x54 and a zero, then
/// forwards (arg0, flag, arg1) — flag is bit 5 of the byte at +0x39 — to a
/// second worker and returns its result. Like its sibling forwarder, the
/// original pushes the flag byte as a word carrying register leftovers; the
/// rewrite passes only the flag bit and the contract masks that argument.
export!(thiscall, rw_008a60b0(this: *mut u8, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let count = *this.add(0x48);
        if count == 0xff {
            return 1;
        }
        let index = *this.add(0x40) as u32;
        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let slot = table
            .wrapping_add(index.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10) as *const u32;
        let target = (*slot).wrapping_add(stride.wrapping_mul(count as u32));
        if target == 0 {
            return 1;
        }
        let field54 = *(this.add(0x54) as *const u32);
        let _: u32 = callee_thiscall!(1, u32, target, field54, 0);
        let flag = ((*this.add(0x39)) >> 5) & 1;
        callee_thiscall!(2, u32, target, arg0, flag as u32, arg1)
    }
});
