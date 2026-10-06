// original: 0x0065B690 fog_control_ctor (proposed)

/// Construct the object: self-link, vtable stamp, sub-object init, constants.
///
/// Links the node at `+8` to itself (both words), stamps the class vtable
/// pointer at `+0`, runs the sub-object callee (thiscall on `this + 0x10`,
/// no stack arguments; its answer is loaded twice and discarded, but a null
/// answer faults both sides identically), then writes the constant fields
/// at `+0x18`, `+0x1c`, `+0x20`, `+0x24`, `+0x28`, `+0x2c`, `+0x30`, `+0x34`, `+0x38`, `+0x3c`, `+0x40`. The aligned-stack zeroing in the original is unobservable
/// scratch. Returns `this` (thiscall, no arguments).
lf_checker_rt::export!(thiscall, rw_0065b690(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xFE2F84;
        const SUB: u32 = 0x10;
        const CALLEE_SUB: u32 = 1;
        let node = this.wrapping_add(8);
        (node as *mut u32).write_unaligned(node);
        ((node + 4) as *mut u32).write_unaligned(node);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let subans: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_SUB, u32, this.wrapping_add(SUB));
        let _ = core::hint::black_box((subans as *const u32).read_unaligned());
        let _ = core::hint::black_box(((subans + 4) as *const u32).read_unaligned());
            ((this + 0x18) as *mut u32).write_unaligned(0x3C23D70A);
            ((this + 0x1c) as *mut u32).write_unaligned(0xFF9C9C9C);
            ((this + 0x20) as *mut u32).write_unaligned(0x40400000);
            ((this + 0x24) as *mut u32).write_unaligned(0x3ECCCCCD);
            ((this + 0x28) as *mut u32).write_unaligned(0x3ECCCCCD);
            ((this + 0x2c) as *mut u32).write_unaligned(0x3DF5C28F);
            ((this + 0x30) as *mut u32).write_unaligned(0x3F7F3B64);
            ((this + 0x34) as *mut u32).write_unaligned(0x42508000);
            ((this + 0x38) as *mut u32).write_unaligned(0xC2420000);
            ((this + 0x3c) as *mut u32).write_unaligned(0x42738000);
            ((this + 0x40) as *mut u8).write_unaligned(0x01);
        this
    }
});
