// original: 0x00962540 stream_slot_acquire
/// Acquire a stream slot by scanning the slot-state table.
///
/// Takes no arguments. Computes `(seed + 1) % slots` from the seed byte at
/// `0x120F298` and the slot count at `0x11F6FFB` (both unsigned; the count
/// is never zero in the proof, as in the game), then scans forward modulo
/// the count while the state byte at `0x11F6FF0` is neither 1 nor 2. Note
/// the count byte aliases state index 11, so a scan can only stop there
/// when the count itself is 1 or 2 (and then the index cannot reach 11). An
/// index of `0xB` or more (unsigned) tail-calls the fatal-error helper;
/// otherwise the pointer at `0x11F6F7C[index]` has its first byte cleared,
/// the state byte is cleared, and the pointer is returned. The fatal path is
/// taken on half the proof trials; its call registers are uncompared.
lf_checker_rt::export!(cdecl, rw_00962540() -> u32 {
    unsafe {
        const SEED: u32 = 0x120f298;
        const COUNT: u32 = 0x11f6ffb;
        const STATE: u32 = 0x11f6ff0;
        const SLOTS: u32 = 0x11f6f7c;
        const FATAL_FROM: u32 = 0x0b;
        let a = (lf_checker_rt::global::<u8>(SEED) as *const u8).read() as u32 + 1;
        let c = (lf_checker_rt::global::<u8>(COUNT) as *const u8).read() as u32;
        let mut idx = a % c;
        let state = |i: u32| {
            (lf_checker_rt::relocated(STATE).wrapping_add(i) as *const u8).read()
        };
        if state(idx) != 2 {
            loop {
                if state(idx) == 1 {
                    break;
                }
                idx = (idx + 1) % c;
                if state(idx) == 2 {
                    break;
                }
            }
        }
        if idx >= FATAL_FROM {
            return lf_checker_rt::callee_cdecl!(1, u32,);
        }
        let e = (lf_checker_rt::relocated(SLOTS).wrapping_add(idx * 4) as *const u32)
            .read_unaligned();
        (lf_checker_rt::relocated(STATE).wrapping_add(idx) as *mut u8).write(0);
        (e as *mut u8).write(0);
        e
    }
});
