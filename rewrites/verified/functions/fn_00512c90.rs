// original: 0x00512c90 leaderboard_race_174_ctor
/// Construct the ranked episodic-race leaderboard info object (race 174).
///
/// Runs the shared base constructor through the object pointer, then installs
/// this class's primary vtable and the inner info vtable, marks the object
/// initialised, resets the row cursor to empty (-1) and clears the trailing
/// info fields. Returns the object pointer.
export!(thiscall, rw_00512c90(this: *mut u8) -> u32 {
    const PRIMARY_VTABLE: u32 = 0x00fd26b0;
    const INNER_VTABLE: u32 = 0x00fd0644;
    const INNER_BASE: usize = 0x4a0;
    const CURSOR_OFF: usize = 0x4a4;
    const FIELDS_OFF: usize = 0x4a8;
    const FIELDS_LEN: usize = 5 * 4;
    const INIT_FLAG_OFF: usize = 0x5a4;
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(PRIMARY_VTABLE);
        *((this.add(INNER_BASE)) as *mut u32) = relocated(INNER_VTABLE);
        *this.add(INIT_FLAG_OFF) |= 1;
        *((this.add(CURSOR_OFF)) as *mut u32) = u32::MAX;
        core::ptr::write_bytes(this.add(FIELDS_OFF), 0, FIELDS_LEN);
        this as u32
    }
});
