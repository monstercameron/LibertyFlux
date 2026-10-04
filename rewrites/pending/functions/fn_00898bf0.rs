// original: 0x00898bf0 audio_forward_indirect
/// Forwards a dword from an indirectly addressed record to the index resolver.
///
/// Loads the record pointer through one level of indirection, reads the dword
/// sitting one byte past its start, and passes it to the resolver, returning
/// the resolver's answer.
export!(stdcall, rw_00898bf0(outer: u32) -> u32 {
    unsafe {
        let inner = core::ptr::read_unaligned(outer as *const u32);
        let value = core::ptr::read_unaligned(inner.wrapping_add(1) as *const u32);
        callee_stdcall!(1, u32, value)
    }
});
