// original: 0x00875290 add_ctor
/// Construct an additive blend motion request.
///
/// Runs the embedded source constructor (stubbed), clears the two child
/// slots at `0x194` and `0x198`, runs the two weight-block clear helpers
/// (stubbed), installs the additive-blend vtable and returns the object.
export!(thiscall, rw_00875290(this: u32) -> u32 {
    unsafe {
        const CHILD_A: usize = 0x194 / 4;
        const CHILD_B: usize = 0x198 / 4;
        let _: u32 = callee_thiscall!(1, u32, this);
        let base = this as *mut u32;
        base.add(CHILD_A).write(0);
        base.add(CHILD_B).write(0);
        let _: u32 = callee_thiscall!(2, u32, this);
        let _: u32 = callee_thiscall!(3, u32, this);
        base.write(relocated(0xFE813C));
        this
    }
});
