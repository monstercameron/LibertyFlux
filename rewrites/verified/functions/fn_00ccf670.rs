// original: 0x00ccf670 melee_dtor
/// Destroy the complex melee object and its fixed member array.
///
/// Writes the vtable pointer, releases the members at `this+0x50` and
/// `this+0xcc` when non-null (thiscall on the member value, member address
/// as argument), runs the reference cleanup, destroys ten fixed slots
/// (`this+0xc0`, `this+0xb4`, seven stepping down by 0xc to `this+0x60`,
/// `this+0x54`), then tail-calls the base destructor and returns its
/// answer. Thiscall, no stack arguments.
export!(thiscall, rw_00ccf670(this: u32) -> u32 {
    unsafe {
        const VT: u32 = 0x00eda884;
        const A_OFF: u32 = 0x50;
        const B_OFF: u32 = 0xcc;
        const FIRST_SLOT: u32 = 0xc0;
        const ARRAY_TOP: u32 = 0xb4;
        const ARRAY_STEP: u32 = 0x0c;
        const ARRAY_EXTRA: u32 = 7;
        const LAST_SLOT: u32 = 0x54;
        (this as *mut u32).write_unaligned(relocated(VT));
        let a = (this.wrapping_add(A_OFF) as *const u32).read_unaligned();
        if a != 0 {
            let _: u32 = callee_thiscall!(1, u32, a, this.wrapping_add(A_OFF));
        }
        let b = (this.wrapping_add(B_OFF) as *const u32).read_unaligned();
        if b != 0 {
            let _: u32 = callee_thiscall!(2, u32, b, this.wrapping_add(B_OFF));
        }
        let _: u32 = callee_thiscall!(3, u32, this);
        let _: u32 = callee_thiscall!(4, u32, this.wrapping_add(FIRST_SLOT));
        let mut slot = this.wrapping_add(ARRAY_TOP);
        let _: u32 = callee_thiscall!(4, u32, slot);
        let mut i = 0u32;
        while i < ARRAY_EXTRA {
            slot = slot.wrapping_sub(ARRAY_STEP);
            let _: u32 = callee_thiscall!(4, u32, slot);
            i += 1;
        }
        let _: u32 = callee_thiscall!(4, u32, this.wrapping_add(LAST_SLOT));
        callee_thiscall!(5, u32, this)
    }
});
