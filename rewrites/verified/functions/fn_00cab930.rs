// original: 0x00CAB930 event_solo_construct (proposed)

/// Construct a standalone event record in place and return its address.
///
/// Runs the sized base constructor on `this`, stores the integer argument at
/// `+0x20`, installs the record's two virtual tables, zeroes the words at
/// `+0x24`, `+0x7C`, `+0x80`, `+0x84`, `+0xA0`, `+0xA4` and `+0xA8`, sets
/// flag bit 0 at `+0xB4`, then runs the record finaliser on `this`.
///
/// Original: 0x00CAB930 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cab930(this: u32, link: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const FINISHER: u32 = 2;
        const VTABLE: u32 = 0x00ED8C2C;
        const VTABLE2: u32 = 0x00ED8C84;
        const BASE_SIZE_BITS: u32 = 0x3F800000;
        const LINK: u32 = 0x20;
        const FLAGS: u32 = 0xB4;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, BASE_SIZE_BITS);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE2));
        (this.wrapping_add(LINK) as *mut u32).write_unaligned(link);
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x7C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x80) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x84) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0xA0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0xA4) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0xA8) as *mut u16).write_unaligned(0);
        let flags = (this.wrapping_add(FLAGS) as *const u32).read_unaligned();
        (this.wrapping_add(FLAGS) as *mut u32).write_unaligned(flags | 1);
        lf_checker_rt::callee_thiscall!(FINISHER, u32, this);
        this
    }
});
