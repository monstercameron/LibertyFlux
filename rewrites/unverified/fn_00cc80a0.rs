// original: 0x00CC80A0 fragInstGta::vf3

/// Returns the auxiliary physics record at `record + 0x1460` when the receiver
/// has flag `0x4000` and the referenced record has kind bits `0x80` and type 3.
/// The record pointer is stored at receiver offset `+0x0c`; kind bits are read
/// from `+0x28` and the type word from `+0x1304`. Every failed condition tail
/// dispatches to the shared thiscall helper with the original receiver. The
/// method has no stack arguments and returns the selected pointer or helper
/// result in EAX.
lf_checker_rt::export!(thiscall, rw_00cc80a0(this: u32) -> u32 {
    const RECEIVER_FLAGS: u32 = 0x0a;
    const RECORD_POINTER: u32 = 0x0c;
    const RECORD_KIND: u32 = 0x28;
    const RECORD_TYPE: u32 = 0x1304;
    const RESULT_FIELD: u32 = 0x1460;
    const REQUIRED_FLAG: u16 = 0x4000;
    const KIND_MASK: u32 = 0x03c0;
    const REQUIRED_KIND: u32 = 0x0080;
    const REQUIRED_TYPE: u32 = 3;
    const FALLBACK: u32 = 1;

    unsafe {
        let fallback = || lf_checker_rt::callee_thiscall!(FALLBACK, u32, this);
        let receiver_flags = (this.wrapping_add(RECEIVER_FLAGS) as *const u16).read_unaligned();
        if receiver_flags & REQUIRED_FLAG == 0 {
            return fallback();
        }
        let record = (this.wrapping_add(RECORD_POINTER) as *const u32).read_unaligned();
        if record == 0 {
            return fallback();
        }
        let kind = (record.wrapping_add(RECORD_KIND) as *const u32).read_unaligned();
        let record_type = (record.wrapping_add(RECORD_TYPE) as *const u32).read_unaligned();
        if kind & KIND_MASK != REQUIRED_KIND || record_type != REQUIRED_TYPE {
            return fallback();
        }
        record.wrapping_add(RESULT_FIELD)
    }
});
