// original: 0x00bc6790 IS_CAR_IN_ANGLED_AREA_3D
/// Script native handler `IS_CAR_IN_ANGLED_AREA_3D`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, arg1 float bits, arg2 float bits, arg3 float bits, arg4 float bits, arg5 float bits, arg6 float bits, arg7 float bits, arg8 bool, calls the engine worker
/// (intercepted by the checker), and writes the zero-extended low byte of the engine answer to the return slot.
///
/// Note: the original reuses its own incoming argument slot as scratch for
/// the bool coercion, clobbering the caller's pushed ctx word; that write is
/// caller-invisible, so this rewrite computes the same pushed value without it.
export!(cdecl, rw_00bc6790(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        let a5 = *args.add(5);
        let a6 = *args.add(6);
        let a7 = *args.add(7);
        // Bool argument: the original coerces it with cmp/setne into its own incoming argument slot, so the pushed dword keeps the slot's high bytes. Only the low byte (arg != 0) is the value; the high bytes alias the caller's ctx word.
        let a8 = ((ctx as u32) & 0xFFFFFF00) | ((*args.add(8) != 0) as u32);
        let ans: u32 = callee_cdecl!(1, u32, a0, a1, a2, a3, a4, a5, a6, a7, a8,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
