// original: 0x00e3a660 notify_input_targets (proposed)

/// Notify every input target of a new input value, then re-sync the first ten.
///
/// `this` points to an object holding thirteen child-object pointers at
/// `+0x1E4..+0x214` (every 4 bytes) and two selector bytes at `+0x239` and
/// `+0x23A`. `arg` is an opaque 32-bit input value passed to every child.
///
/// Phase one calls virtual slot `+0x120` on eleven children in order: the
/// child at `+0x1E8`, then `+0x1EC` when the byte at `+0x23A` is non-zero
/// else `+0x1E4`, then `+0x1F0` and `+0x1F4`, then `+0x1F8` when the byte at
/// `+0x239` is non-zero else `+0x1FC`, then `+0x200` through `+0x214`.
/// Phase two calls virtual slot `+0x28` on the ten children at `+0x1E4`
/// through `+0x208` in order. Every call passes `arg` as its one stack
/// argument with the child in ECX (thiscall/1); the callees pop it.
///
/// Nothing is read besides `this`, the thirteen pointers, the two selector
/// bytes and `arg`; nothing is written anywhere; no answer is used except
/// that the last call's return value is left in EAX.
///
/// Original: 0x00E3A660 (thiscall, one stack word; 22 indirect call sites).
lf_checker_rt::export!(thiscall, rw_00e3a660(this: u32, arg: u32) -> u32 {
    unsafe {
        const FIRST_CHILD: u32 = 0x1E4;
        const FLAG_A: u32 = 0x239;
        const FLAG_B: u32 = 0x23A;
        const SLOT_NOTIFY: u32 = 0x120;
        const SLOT_SYNC: u32 = 0x28;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn call_child(child: u32, slot: u32, arg: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(child) + slot) as usize);
                f(child, arg)
            }
        }

        let mut last = 0u32;
        // Phase one: eleven notify calls with two flag-selected children.
        let sel_b = if rd8(this + FLAG_B) != 0 { 0x1EC } else { 0x1E4 };
        let sel_a = if rd8(this + FLAG_A) != 0 { 0x1F8 } else { 0x1FC };
        for off in [0x1E8u32, sel_b, 0x1F0, 0x1F4, sel_a, 0x200, 0x204, 0x208, 0x20C, 0x210, 0x214] {
            last = call_child(rd32(this + off), SLOT_NOTIFY, arg);
        }
        // Phase two: ten sync calls over the first ten children in order.
        let mut off = FIRST_CHILD;
        while off <= 0x208 {
            last = call_child(rd32(this + off), SLOT_SYNC, arg);
            off += 4;
        }
        last
    }
});
