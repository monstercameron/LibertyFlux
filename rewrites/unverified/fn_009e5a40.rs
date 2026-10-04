// original: 0x009e5a40 ped_stamp_value (proposed)

/// Record a value together with the current global tick on a ped object.
///
/// Writes the argument word to `this + 0xa64`, copies the global tick
/// counter to `this + 0xa68`, sets the valid flag byte at `this + 0xa6c`
/// and returns the tick (`thiscall`, one stack word).
lf_checker_rt::export!(thiscall, rw_009e5a40(this: u32, stamp: u32) -> u32 {
    unsafe {
        const VALUE: u32 = 0xa64;
        const STAMP: u32 = 0xa68;
        const FLAG: u32 = 0xa6c;
        const COUNTER: u32 = 0x011735b4;
        ((this + VALUE) as *mut u32).write_unaligned(stamp);
        let now = lf_checker_rt::global::<u32>(COUNTER).read();
        ((this + STAMP) as *mut u32).write_unaligned(now);
        ((this + FLAG) as *mut u8).write(1);
        now
    }
});
