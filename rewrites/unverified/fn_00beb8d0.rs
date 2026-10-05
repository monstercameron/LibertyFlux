// original: 0x00beb8d0 set_id_byte_zero_rest
/// Store an id byte and zero the seven bytes after it.
///
/// Writes the low byte of the argument to `[this]` and zeroes bytes 1-7
/// (a byte, a dword and a word store). The low byte of EAX on return is the
/// id byte; the upper bytes keep the caller's EAX, so the contract compares
/// only AL. Thiscall, one stack argument.
export!(thiscall, rw_00beb8d0(this: u32, arg: u32) -> u32 {
    unsafe {
        ((this) as *mut u8).write(arg as u8);
        ((this + 1) as *mut u8).write(0);
        ((this + 2) as *mut u32).write_unaligned(0);
        ((this + 6) as *mut u16).write_unaligned(0);
        arg & 0xff
    }
});
