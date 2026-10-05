// original: 0x00937B60 stream_obj_zero_init_b (proposed)

/// Zero four fields of a small streaming record, returning the record.
///
/// `this` points to the record. Writes dword 0 at `+0x00` and `+0x104`,
/// word 0 at `+0x108` and byte 0 at `+0x04`; all other bytes are untouched.
/// Returns `this` (thiscall).
lf_checker_rt::export!(thiscall, rw_00937b60(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const MARK: u32 = 0x04;
        const COUNT: u32 = 0x104;
        const SHORT: u32 = 0x108;
        ((this + HEAD) as *mut u32).write_unaligned(0);
        ((this + COUNT) as *mut u32).write_unaligned(0);
        ((this + SHORT) as *mut u16).write_unaligned(0);
        ((this + MARK) as *mut u8).write(0);
        this
    }
});
