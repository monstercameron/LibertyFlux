// original: 0x008B1C10 audio_filter_init (proposed)

/// Initialise a filter object: vtable slot, three defaulted coefficient
/// blocks, and two state words. Returns `this`.
///
/// Layout written (all offsets from `this`):
/// * `+0x00`: vtable pointer constant `0xe7cdd8`.
/// * Three identical blocks at `+0x78`, `+0x8c`, `+0xa0`, each 20 bytes:
///   floats `1.0, 0.5, 0.5`, a zero dword, a zero word (two padding bytes at
///   the block end are left untouched).
/// * `+0xb8`: 1, `+0xb4`: 0 (the original stores `+0xb8` first; the two
///   addresses are distinct so the order is unobservable).
/// No calls. Original is thiscall with no stack words (plain `ret`).
lf_checker_rt::export!(thiscall, rw_008B1C10(this: u32) -> u32 {
    const VTABLE_FILE_VA: u32 = 0x00e7_cdd8;
    let vtable = lf_checker_rt::relocated(VTABLE_FILE_VA);
    const BLOCKS: [u32; 3] = [0x78, 0x8c, 0xa0];
    const ONE: u32 = 0x3f80_0000;
    const HALF: u32 = 0x3f00_0000;
    unsafe {
        (this as *mut u32).write_unaligned(vtable);
        for b in BLOCKS {
            ((this + b) as *mut u32).write_unaligned(ONE);
            ((this + b + 4) as *mut u32).write_unaligned(HALF);
            ((this + b + 8) as *mut u32).write_unaligned(HALF);
            ((this + b + 12) as *mut u32).write_unaligned(0);
            ((this + b + 16) as *mut u16).write_unaligned(0);
        }
        ((this + 0xb8) as *mut u32).write_unaligned(1);
        ((this + 0xb4) as *mut u32).write_unaligned(0);
    }
    this
});
