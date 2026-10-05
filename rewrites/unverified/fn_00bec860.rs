// original: 0x00bec860 solve_then_store_164
/// Run the three-argument solver, then copy `[this+0x14]` to `[dst+0x164]`.
///
/// Calls the solver (thiscall on `this` with three stack args; intercepted,
/// answer ignored): the args are `dst`, `sel` and, when `sel` is nonzero,
/// the float bits of `f`, else 0. Then stores `[this+0x14]` at
/// `[dst+0x164]` and returns the copied word. Thiscall, three stack
/// arguments.
export!(thiscall, rw_00bec860(this: u32, dst: u32, sel: u32, fbits: u32) -> u32 {
    unsafe {
        const SOLVER: u32 = 1;
        const SRC_OFF: u32 = 0x14;
        const DST_OFF: u32 = 0x164;
        if sel == 0 {
            let _: u32 = callee_thiscall!(SOLVER, u32, this, dst, sel, 0);
        } else {
            let _: u32 = callee_thiscall!(SOLVER, u32, this, dst, sel, fbits);
        }
        let v = ((this + SRC_OFF) as *const u32).read_unaligned();
        ((dst + DST_OFF) as *mut u32).write_unaligned(v);
        v
    }
});
