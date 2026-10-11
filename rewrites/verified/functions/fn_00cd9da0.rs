// original: 0x00CD9DA0 CTaskSimpleNM::CTaskSimpleNM

/// Initialize the compact `CTaskSimpleNM` base state after its base-task
/// constructor. The byte at 0x14 has its low three bits cleared; the word at
/// 0x18 has its low half set to all ones while its high half is preserved.
/// The word at 0x1A is set to 0xFFFF, the two incoming words are truncated to
/// halfwords at 0x20 and 0x22, and the byte at 0x1C is set to 2 before its
/// following dword OR sets bytes 0x1D and 0x1E to 0xFF while preserving byte
/// 0x1F. It installs the vtable, clears the dword at 0x24 and returns `this`.
///
/// Calling convention: thiscall with two 32-bit stack words, of which only
/// their low halfwords are stored. The base constructor is thiscall with no
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_00cd9da0(this: u32, first_halfword: u32, second_halfword: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const PREFIX_BYTE: u32 = 0x14;
    const PREFIX_MASK: u8 = 0xF8;
    const BASE_FLAGS: u32 = 0x18;
    const LOW_FLAGS_MASK: u32 = 0xFFFF;
    const INITIALIZED_HALFWORD: u32 = 0x1A;
    const FIRST_VALUE: u32 = 0x20;
    const SECOND_VALUE: u32 = 0x22;
    const MODE_DWORD: u32 = 0x1C;
    const MODE_BYTE: u8 = 0x02;
    const MODE_MASK: u32 = 0x00FF_FF00;
    const VTABLE: u32 = 0x00ED_CF84;
    const EMPTY_FIELD: u32 = 0x24;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
        let flags_pointer = (this + BASE_FLAGS) as *mut u32;
        let flags = flags_pointer.read_unaligned();
        flags_pointer.write_unaligned(flags | LOW_FLAGS_MASK);
        let prefix_pointer = (this + PREFIX_BYTE) as *mut u8;
        prefix_pointer.write(prefix_pointer.read() & PREFIX_MASK);
        ((this + INITIALIZED_HALFWORD) as *mut u16).write_unaligned(u16::MAX);
        ((this + FIRST_VALUE) as *mut u16).write_unaligned(first_halfword as u16);
        ((this + SECOND_VALUE) as *mut u16).write_unaligned(second_halfword as u16);
        let mode_pointer = (this + MODE_DWORD) as *mut u32;
        mode_pointer.write_unaligned((mode_pointer.read_unaligned() & !0xFF) | MODE_BYTE as u32);
        let mode = mode_pointer.read_unaligned();
        mode_pointer.write_unaligned(mode | MODE_MASK);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + EMPTY_FIELD) as *mut u32).write_unaligned(0);
        this
    }
});
