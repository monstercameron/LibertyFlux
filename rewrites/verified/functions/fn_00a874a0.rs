// original: 0x00A874A0 render_phase_construct
/// Construct the phase object: install its vtable and initialize members.
///
/// Installs the vtable pointer, constructs the embedded member objects
/// (one at slot 0x50, two alike at 0xB0 and 0x4A0, three alike at
/// 0x8D0/0x8DC/0x8E8 twice), zeroes the flag array at 0x890..0x8CD, sets the
/// tag words, and when the source argument is non-null initializes the two
/// alike members from it (source + 0x10) and raises the ready flag.
/// Returns the object pointer.
export!(thiscall, rw_a874a0(this: u32, source: u32) -> u32 {
    const VTABLE: u32 = 0x00EA_22D4;
    const FLAG_MASK: u8 = 0xF9;
    unsafe {
        (this as *mut u32).write(relocated(VTABLE));
    }
    callee_thiscall!(1, u32, this.wrapping_add(0x50));
    callee_thiscall!(2, u32, this.wrapping_add(0xB0));
    callee_thiscall!(2, u32, this.wrapping_add(0x4A0));
    callee_thiscall!(3, u32, this.wrapping_add(0x8D0));
    callee_thiscall!(3, u32, this.wrapping_add(0x8DC));
    callee_thiscall!(3, u32, this.wrapping_add(0x8E8));
    unsafe {
        // Eight (dword, byte) flag pairs; the padding bytes between them
        // are left untouched.
        for i in 0..8u32 {
            ((this + 0x890 + i * 8) as *mut u32).write(0);
            ((this + 0x894 + i * 8) as *mut u8).write(0);
        }
        let flags = (this + 0x19) as *mut u8;
        flags.write(flags.read() & FLAG_MASK);
        ((this + 0x1C) as *mut u16).write(0);
        ((this + 0x8F8) as *mut u32).write(u32::MAX);
        ((this + 0x8FC) as *mut u32).write(u32::MAX);
        ((this + 0x1B) as *mut u8).write(0);
        ((this + 0x938) as *mut u32).write(u32::MAX);
        ((this + 0x8F4) as *mut u32).write(1);
    }
    callee_thiscall!(3, u32, this.wrapping_add(0x8D0));
    callee_thiscall!(3, u32, this.wrapping_add(0x8DC));
    callee_thiscall!(3, u32, this.wrapping_add(0x8E8));
    if source != 0 {
        let from = source.wrapping_add(0x10);
        callee_thiscall!(4, u32, this.wrapping_add(0xB0), from);
        unsafe { ((this + 0x1C) as *mut u8).write(1) };
        callee_thiscall!(4, u32, this.wrapping_add(0x4A0), from);
    }
    unsafe {
        ((this + 0x1E) as *mut u16).write(0);
        ((this + 0x20) as *mut u8).write(0);
        ((this + 0x1A) as *mut u8).write(0);
        ((this + 0x34) as *mut u32).write(0);
        ((this + 0x17) as *mut u16).write(0);
        ((this + 0x14) as *mut u8).write(0);
        ((this + 0x3C) as *mut u32).write(2);
        ((this + 0x40) as *mut u32).write(3);
        ((this + 0x44) as *mut u8).write(0);
        ((this + 0x10) as *mut u32).write(0);
    }
    this
});
