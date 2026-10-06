// original: 0x0065B450 atmospheric_scattering_ctor (proposed)

/// Construct the object: self-link, vtable stamp, sub-object init, constants.
///
/// Links the node at `+8` to itself (both words), stamps the class vtable
/// pointer at `+0`, runs the sub-object callee (thiscall on `this + 0x60`,
/// no stack arguments; its answer is loaded twice and discarded, but a null
/// answer faults both sides identically), then writes the constant fields
/// at `+0x10`, `+0x14`, `+0x18`, `+0x68`, `+0x6c`. The aligned-stack zeroing in the original is unobservable
/// scratch. Returns `this` (thiscall, no arguments).
lf_checker_rt::export!(thiscall, rw_0065b450(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xFE3010;
        const SUB: u32 = 0x60;
        const CALLEE_SUB: u32 = 1;
        let node = this.wrapping_add(8);
        (node as *mut u32).write_unaligned(node);
        ((node + 4) as *mut u32).write_unaligned(node);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let subans: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_SUB, u32, this.wrapping_add(SUB));
        let _ = core::hint::black_box((subans as *const u32).read_unaligned());
        let _ = core::hint::black_box(((subans + 4) as *const u32).read_unaligned());
            ((this + 0x10) as *mut u32).write_unaligned(0x00000000);
            ((this + 0x14) as *mut u32).write_unaligned(0x3F333333);
            ((this + 0x18) as *mut u32).write_unaligned(0x3F333333);
            ((this + 0x68) as *mut u32).write_unaligned(0x00000000);
            ((this + 0x6c) as *mut u8).write_unaligned(0x01);
        this
    }
});
