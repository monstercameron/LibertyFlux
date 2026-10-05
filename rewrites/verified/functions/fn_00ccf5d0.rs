// original: 0x00ccf5d0 melee_move_ctor
/// Construct the melee-move object: base, vtables, constants, registration.
///
/// Runs the base constructor (thiscall, argument 1), clears bit 0 of the
/// flag byte at `this+0x30`, writes the two vtable pointers and the float
/// constants (1.0, 0.2, 1.5) into the two parameter blocks at `+0x20` and
/// `+0x34`, zeroes the state words, sets bits 0 and 2 of the mode byte at
/// `this+0x68` while clearing bit 1, zeroes `+0x60`/`+0x64`, then runs the
/// registration helper (thiscall, the two stack arguments) and returns
/// `this`. Thiscall with two stack arguments.
export!(thiscall, rw_00ccf5d0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const VT_MAIN: u32 = 0x00eda8dc;
        const VT_SECOND: u32 = 0x00eda930;
        const ONE: u32 = 0x3f800000;
        const FIFTH: u32 = 0x3e4ccccd;
        const THREE_HALVES: u32 = 0x3fc00000;
        let _: u32 = callee_thiscall!(1, u32, this, 1);
        let f30 = (this.wrapping_add(0x30) as *const u8).read();
        (this.wrapping_add(0x30) as *mut u8).write(f30 & 0xfe);
        (this as *mut u32).write_unaligned(relocated(VT_MAIN));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(relocated(VT_SECOND));
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(ONE);
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(FIFTH);
        (this.wrapping_add(0x2c) as *mut u32).write_unaligned(THREE_HALVES);
        (this.wrapping_add(0x34) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x38) as *mut u32).write_unaligned(ONE);
        (this.wrapping_add(0x3c) as *mut u32).write_unaligned(FIFTH);
        (this.wrapping_add(0x40) as *mut u32).write_unaligned(THREE_HALVES);
        (this.wrapping_add(0x50) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x54) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x58) as *mut u32).write_unaligned(0);
        let m68 = (this.wrapping_add(0x68) as *const u8).read();
        (this.wrapping_add(0x60) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x64) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x68) as *mut u8).write((m68 & 0xfd) | 5);
        let _: u32 = callee_thiscall!(2, u32, this, a0, a1);
        this
    }
});
