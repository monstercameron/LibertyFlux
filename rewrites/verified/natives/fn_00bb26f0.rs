// original: 0x00BB26F0 MAINTAIN_FLASHING_STAR_AFTER_OFFENCE
/// Script native handler `MAINTAIN_FLASHING_STAR_AFTER_OFFENCE`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, arg1 bool, calls the engine worker
/// (intercepted by the checker), and returns nothing to the script (the engine answer stays in EAX).
///
/// Note: the original reuses its own incoming argument slot as scratch for
/// the bool coercion, clobbering the caller's pushed ctx word; that write is
/// caller-invisible, so this rewrite computes the same pushed value without it.
export!(cdecl, rw_00bb26f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        // Bool argument: the original coerces it with cmp/setne into its own incoming argument slot, so the pushed dword keeps the slot's high bytes. Only the low byte (arg != 0) is the value; the high bytes alias the caller's ctx word.
        let a1 = ((*args.add(1) != 0) as u32);
        let ans: u32 = callee_cdecl!(1, u32, a0, a1,);
        ans
    }
});
