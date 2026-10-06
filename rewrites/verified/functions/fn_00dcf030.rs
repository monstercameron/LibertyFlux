// original: 0x00dcf030 csv_field_ctor (proposed)

/// Constructor for a csv field descriptor: installs the vtable and stores
/// the tag, the four float bounds and a zeroed state byte.
///
/// `this` points to the new object. The vtable goes at `+0x0`, the integer
/// tag at `+0x4`, the four floats at `+0x8`, `+0xc`, `+0x10` and `+0x14`
/// (copied bitwise; the original moves them with `movss`, no arithmetic),
/// and a zero byte at `+0x18`. Returns `this`.
///
/// Original: 0x00DCF030 (thiscall, five stack words, returns the object).
lf_checker_rt::export!(thiscall, rw_00dcf030(this: u32, f0: u32, f1: u32, f2: u32, f3: u32, n: u32) -> u32 {
    unsafe {
        /// Vtable installed by this constructor.
        const VTABLE: u32 = 0xeea88c;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 4) as *mut u32).write_unaligned(n);
        // Four float slots copied bitwise (movss, no arithmetic).
        ((this + 8) as *mut u32).write_unaligned(f0);
        ((this + 0xc) as *mut u32).write_unaligned(f1);
        ((this + 0x10) as *mut u32).write_unaligned(f2);
        ((this + 0x14) as *mut u32).write_unaligned(f3);
        ((this + 0x18) as *mut u8).write(0);
        this
    }
});
