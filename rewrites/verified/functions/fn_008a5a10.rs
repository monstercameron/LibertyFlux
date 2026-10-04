// original: 0x008a5a10 aud_math_op_forward_flagged
/// Forward two arguments to a worker selected through the entity table.
///
/// Extracts a flag (bit 5 of the byte at +0x39) and resolves a target pointer
/// from the audio entity table when the count byte at +0x48 is not 0xff,
/// else targets null. Calls the worker with (arg0, flag, arg1) and returns
/// its result. Note: the original pushes its byte local as a full word, so
/// the flag word's upper bytes carry the incoming register value; the rewrite
/// passes only the flag bit and the contract masks that argument.
export!(thiscall, rw_008a5a10(this: *mut u8, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let flag = (*this.add(0x39) >> 5) & 1;
        let count = *this.add(0x48);
        let target = if count == 0xff {
            0
        } else {
            let index = *this.add(0x40) as u32;
            let stride = *global::<u32>(0x115d964);
            let table = *global::<u32>(0x115d988);
            let slot = table
                .wrapping_add(index.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32;
            (*slot).wrapping_add(stride.wrapping_mul(count as u32))
        };
        callee_thiscall!(1, u32, target, arg0, flag as u32, arg1)
    }
});
