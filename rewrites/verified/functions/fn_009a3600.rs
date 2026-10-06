// original: 0x009A3600 audio_entity_construct (proposed)

/// Construct an audio entity in place.
///
/// Runs the base constructor (callee 1, thiscall/0), installs the two
/// relocated table addresses at +0 and +8, clears the state word at +0xC,
/// and constructs the two embedded members at +0x30 and +0x4C (callee 2,
/// thiscall/0, twice with the member addresses in `ecx`). Returns the
/// object pointer. Thiscall with no stack words.
lf_checker_rt::export!(thiscall, rw_009A3600(this: u32) -> u32 {
    unsafe {
        const TABLE_MAIN: u32 = 0x00E91480;
        const TABLE_INNER: u32 = 0x00E908AC;
        const INNER: u32 = 8;
        const STATE: u32 = 0x0C;
        const MEMBER_A: u32 = 0x30;
        const MEMBER_B: u32 = 0x4C;
        const BASE_CTOR: u32 = 1;
        const MEMBER_CTOR: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TABLE_MAIN));
        ((this.wrapping_add(INNER)) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(TABLE_INNER));
        ((this.wrapping_add(STATE)) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, this.wrapping_add(MEMBER_A));
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, this.wrapping_add(MEMBER_B));
        this
    }
});
