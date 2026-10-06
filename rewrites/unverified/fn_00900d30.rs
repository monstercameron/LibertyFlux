// original: 0x00900d30 input_state_init (proposed)
/// Initialise an input state object with zeros, ones and a scratch float.
///
/// `this` points to the object. The kind byte at `+8` is set to 1, the
/// halfword at `+0x20` and the words at `+0x24`, `+0x30`-`+0x38`,
/// `+0x40`-`+0x4c`, `+0x54` and `+0x5c` and the byte at `+0x58` are zeroed,
/// the word at `+0x50` is set to 1.0, and the word at `+0x3c` receives a
/// float the original reads from below its own frame (uninitialized caller
/// stack; proven as zero under a zero stack fill). Returns `this`.
/// Thiscall with no stack arguments.
export!(thiscall, rw_00900d30(this: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x08;
        const HALF_OFF: u32 = 0x20;
        const W24_OFF: u32 = 0x24;
        const FLOAT_OFF: u32 = 0x3C;
        const ONE_OFF: u32 = 0x50;
        const FLAG_OFF: u32 = 0x58;
        ((this.wrapping_add(HALF_OFF)) as *mut u16).write_unaligned(0);
        ((this.wrapping_add(W24_OFF)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(KIND_OFF)) as *mut u8).write(1);
        for off in [0x30u32, 0x34, 0x38] {
            ((this.wrapping_add(off)) as *mut u32).write_unaligned(0);
        }
        // Below-frame scratch float: zero under the contract's stack fill.
        ((this.wrapping_add(FLOAT_OFF)) as *mut u32).write_unaligned(0);
        for off in [0x40u32, 0x44, 0x48, 0x4c] {
            ((this.wrapping_add(off)) as *mut u32).write_unaligned(0);
        }
        ((this.wrapping_add(0x54)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(FLAG_OFF)) as *mut u8).write(0);
        ((this.wrapping_add(0x5c)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(ONE_OFF)) as *mut u32).write_unaligned(0x3F800000);
        this
    }
});
