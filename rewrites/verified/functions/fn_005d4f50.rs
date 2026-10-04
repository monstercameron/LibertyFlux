// original: 0x005d4f50 cped_create_bound_object
/// Build the sub-object bound at slot `+0x6c` and initialise it.
///
/// Asks the manager (reached through a global) for an instance; when one is
/// available, creates the bound object from it with the four forwarded
/// arguments, stores it in the slot and initialises it with the fifth
/// argument. Otherwise clears the slot and still runs the initialiser with a
/// null receiver. Returns the slot contents.
export!(thiscall, rw_005d4f50(this_ptr: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let mgr = unsafe { global::<u32>(0x018B62DC).read() };
    let outer: u32 = callee_thiscall!(0, u32, mgr);
    let slot = (this_ptr + 0x6C) as *mut u32;
    if outer != 0 {
        let mid: u32 = callee_thiscall!(1, u32, outer, this_ptr, 1, a1, a2, a3, a4);
        unsafe { slot.write(mid) };
        let _: u32 = callee_thiscall!(2, u32, mid, a5);
        unsafe { slot.read() }
    } else {
        unsafe { slot.write(0) };
        let _: u32 = callee_thiscall!(2, u32, 0, a5);
        unsafe { slot.read() }
    }
});
