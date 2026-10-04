// original: 0x00a7daa0 CSequenceTaskInfo::CSequenceTaskInfo
/// Initialise a sequence task info: run the base constructor, store the
/// step count and three flag bytes, then install this class's vtable.
export!(thiscall, rw_00a7daa0(this: u32, step_count: u32, flag_a: u32, flag_b: u32, flag_c: u32) -> u32 {
    const VTABLE: u32 = 0x00EA12CC;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj.add(0x14) as *mut u32).write_unaligned(step_count);
        *obj.add(0x18) = flag_a as u8;
        *obj.add(0x19) = flag_b as u8;
        *obj.add(0x1a) = flag_c as u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
    }
    this
});
