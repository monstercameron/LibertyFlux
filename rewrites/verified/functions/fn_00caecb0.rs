// original: 0x00caecb0 task_refs_release (proposed)
/// Release the two referenced objects held at `+0xAC`/`+0xB0`, if any.
///
/// For each slot in order: when it holds a non-null pointer, passes it to the
/// release helper (intercepted callee 1, cdecl/1) and clears the slot to null.
/// Takes no stack arguments and returns nothing meaningful. Original is
/// thiscall(`this`).
lf_checker_rt::export!(thiscall, rw_00caecb0(this: u32) -> u32 {
    unsafe {
        const RELEASE: u32 = 1;
        for off in [0xACu32, 0xB0] {
            let slot = (this.wrapping_add(off)) as *mut u32;
            let held = slot.read_unaligned();
            if held != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, held);
                slot.write_unaligned(0);
            }
        }
        0
    }
});
