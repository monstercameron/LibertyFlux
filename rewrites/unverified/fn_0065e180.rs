// original: 0x0065e180 initialise_descriptor_slots
use lf_checker_rt::export;

/// Clear the descriptor's scalar fields and three adjacent slot counters,
/// mark each slot present, and return the descriptor pointer. The pointer is
/// passed in ECX; the layout is the original 32-bit descriptor layout.
export!(thiscall, rw_0065e180(this: *mut u8) -> *mut u8 {
    const ZERO_WORDS: [usize; 11] = [
        0x00, 0x04, 0x08, 0x0C, 0x10, 0x18,
        0x20, 0x24, 0x28, 0x2C, 0x30,
    ];
    const SLOT_FLAGS: usize = 0x38;
    const SLOT_COUNT: usize = 3;

    unsafe {
        for offset in ZERO_WORDS {
            this.add(offset).cast::<u32>().write_unaligned(0);
        }
        this.add(0x34).cast::<u32>().write_unaligned(0);
        for slot in 0..SLOT_COUNT {
            this.add(SLOT_FLAGS + slot).write(1);
        }
        this.add(0x3C).cast::<u32>().write_unaligned(0);
        this.add(0x40).cast::<u32>().write_unaligned(0);
    }
    this
});
