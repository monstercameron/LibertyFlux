// original: 0x00696340 rage::crAnimChannel::eval_indexed

/// Evaluates a channel at an index by virtual dispatch.
///
/// `this` is the channel object; the stack arguments are an integer
/// selector, a float position `t` and an unused word. The evaluator is
/// the virtual slot 9 (`[vtable+0x24]`, reached through the object and
/// called exactly like the original, landing on the same planted stub):
/// thiscall with the object, the selector, the `t` bits (passed through
/// untouched) and an out-slot. The out-slot is the original's own pushed
/// frame word, primed with `this` (the pointer argument is skipped and
/// the slot snapshotted at the call); the callee's word lands there. The
/// result is loaded with `fld` and returned on the x87 stack, compared
/// bit-exactly through the x87 state check (80-bit extended values).
///
/// Original: 0x00696340 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_00696340(this: u32, sel: u32, t_bits: u32, _unused: u32) -> f32 {
    unsafe {
        const EVAL: u32 = 1;
        const VT_SLOT: u32 = 0x24;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vt = rd32(this);
        let eval: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt + VT_SLOT) as usize) };
        let mut slot: u32 = this;
        let _ = eval(this, sel, t_bits, core::ptr::addr_of_mut!(slot) as u32);
        f32::from_bits(slot)
    }
});
