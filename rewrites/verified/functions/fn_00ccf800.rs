// original: 0x00ccf800 melee_move_dtor
/// Destroy the melee-move object: set base vtables, release two members.
///
/// Writes the two vtable pointers, releases each non-null member at
/// `this+0x20` and `this+0x34` through the member destructor (thiscall on
/// the member value, the member's address as argument), then tail-calls the
/// base destructor and returns its answer. Thiscall, no stack arguments.
export!(thiscall, rw_00ccf800(this: u32) -> u32 {
    unsafe {
        const VT_MAIN: u32 = 0x00eda8dc;
        const VT_SECOND: u32 = 0x00eda930;
        const SECOND_OFF: u32 = 0x14;
        const A_OFF: u32 = 0x20;
        const B_OFF: u32 = 0x34;
        (this as *mut u32).write_unaligned(relocated(VT_MAIN));
        (this.wrapping_add(SECOND_OFF) as *mut u32).write_unaligned(relocated(VT_SECOND));
        let a = (this.wrapping_add(A_OFF) as *const u32).read_unaligned();
        if a != 0 {
            let _: u32 = callee_thiscall!(1, u32, a, this.wrapping_add(A_OFF));
        }
        let b = (this.wrapping_add(B_OFF) as *const u32).read_unaligned();
        if b != 0 {
            let _: u32 = callee_thiscall!(2, u32, b, this.wrapping_add(B_OFF));
        }
        callee_thiscall!(3, u32, this)
    }
});
