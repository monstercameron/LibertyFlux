// original: 0x00d75ed0 accumulate_float_pair_forward
// Float-pair accumulate and conditional forward.
// Adds the two argument floats to the two floats at `p`, stashes the sums
// in frame scratch, and when the flag word at this+4 is set, notifies the
// sub-object at this+4 and forwards the argument block plus the sums
// through two callees. The original passes pointers into its own stack
// frame; the rewrite builds equivalent scratch buffers (call_skip + snap).
// Returns the second callee's answer on the taken path, `p` otherwise.
export!(thiscall, rw_00d75ed0(this_obj: u32, p: u32, f1: f32, f2: f32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, 7, 1);
        let p0 = *(p as *const f32);
        let p1 = *((p.wrapping_add(4)) as *const f32);
        let s0 = p0 + f1;
        let s1 = p1 + f2;
        if *((this_obj.wrapping_add(4)) as *const u32) == 0 {
            return p;
        }
        callee_thiscall!(2, u32, this_obj.wrapping_add(4));
        let arg_block = [p, f1.to_bits(), f2.to_bits()];
        let mid = callee_cdecl!(3, u32, arg_block.as_ptr() as u32, 3, 0x40);
        let sum_block = [p0.to_bits(), p1.to_bits(), s0.to_bits(), s1.to_bits()];
        callee_cdecl!(4, u32, sum_block.as_ptr() as u32, mid)
    }
});
