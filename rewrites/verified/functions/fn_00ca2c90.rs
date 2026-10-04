// original: 0x00ca2c90 CEventDamage::~CEventDamage__deleting

/// Deleting destructor: destroy the event and free it on request.
///
/// Runs the destructor (callee 1); when the flag word's bit 0 is set the
/// event is handed to the heap free helper (callee 2, with the global heap)
/// and the flag is consumed. Returns `this`.
///
/// Original: 0x00ca2c90 (thiscall, one stack word = flags).
lf_checker_rt::export!(thiscall, rw_00ca2c90(this: u32, flags: u32) -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            lf_checker_rt::callee_thiscall!(
                2,
                u32,
                *lf_checker_rt::global::<u32>(0x0166_69c8),
                this
            );
        }
        this
    }
});
