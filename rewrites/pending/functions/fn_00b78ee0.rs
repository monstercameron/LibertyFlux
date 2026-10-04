// original: 0x00b78ee0 task_timed_ctor
/// Construct a timed task from an integer selector.
///
/// Runs the simple-task constructor, converts the integer argument to a
/// float through a helper call, stores it into the embedded subobject at
/// offset `0x18`, clears its flag bit, and installs the derived virtual
/// tables. Returns the object pointer.
export!(thiscall, rw_00b78ee0(this: u32, arg: u32) -> u32 {
    let _: u32 = callee_thiscall!(1, u32, this);
    let f: f32 = callee_cdecl!(2, f32, arg);
    unsafe {
        let base = this as *mut u32;
        *base.add(0x14 / 4) = relocated(0x00EB2B98);
        *(base.add(0x18 / 4) as *mut f32) = f;
        *base.add(0x1c / 4) &= !1;
        *base = relocated(0x00EB2CCC);
        *base.add(0x14 / 4) = relocated(0x00EB2D20);
    }
    this
});
