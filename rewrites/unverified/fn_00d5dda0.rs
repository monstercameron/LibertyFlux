// original: 0x00D5DDA0 prt_sys_entity_initialize

/// Initialize a particle system entity after its base construction.
///
/// The `this` pointer addresses an entity with the base flag word at byte
/// offset `0x28`, an auxiliary word at `0x24`, a field at `0x74`, and a byte
/// state at `0x63`. The incoming stack argument is forwarded to the derived
/// setup helper. Construction first invokes the base initializer, then replaces
/// the object's vtable, clears the auxiliary field, normalizes selected flag
/// bits, invokes the setup helper, sets the initialized bit, and fills the byte
/// state with `0xff`. The method uses the 32-bit MSVC thiscall convention,
/// returns `this`, and cleans its one stack argument.
lf_checker_rt::export!(thiscall, rw_prt_sys_entity_initialize(
    this_ptr: u32,
    setup_argument: u32
) -> u32 {
    unsafe {
        const ENTITY_VTABLE: u32 = 0x00EE88BC;
        const FLAGS_OFFSET: u32 = 0x28;
        const AUXILIARY_OFFSET: u32 = 0x74;
        const AUXILIARY_FLAGS_OFFSET: u32 = 0x24;
        const BYTE_STATE_OFFSET: u32 = 0x63;
        const CLEARED_FLAG_MASK: u32 = 0x00000140;
        const REQUIRED_FLAG_BITS: u32 = 0x00000280;
        const INITIALIZED_BIT: u32 = 0x00001000;
        const BASE_INITIALIZER: u32 = 1;
        const DERIVED_SETUP: u32 = 2;

        // The original dispatches both helpers directly. The contract replaces
        // these call sites with scripted stand-ins on both sides.
        let _ = lf_checker_rt::callee_thiscall!(BASE_INITIALIZER, u32, this_ptr);

        let flags_ptr = lf_checker_rt::relocated(this_ptr).wrapping_add(FLAGS_OFFSET) as *mut u32;
        let flags = unsafe { flags_ptr.read_unaligned() };
        let normalized_flags = (flags & !CLEARED_FLAG_MASK) | REQUIRED_FLAG_BITS;

        let vtable_ptr = lf_checker_rt::relocated(this_ptr) as *mut u32;
        unsafe { vtable_ptr.write_unaligned(lf_checker_rt::relocated(ENTITY_VTABLE)) };
        let auxiliary_ptr = lf_checker_rt::relocated(this_ptr).wrapping_add(AUXILIARY_OFFSET) as *mut u32;
        unsafe { auxiliary_ptr.write_unaligned(0) };
        unsafe { flags_ptr.write_unaligned(normalized_flags) };

        let _ = lf_checker_rt::callee_thiscall!(DERIVED_SETUP, u32, this_ptr, setup_argument);

        let auxiliary_flags_ptr =
            lf_checker_rt::relocated(this_ptr).wrapping_add(AUXILIARY_FLAGS_OFFSET) as *mut u32;
        let auxiliary_flags = unsafe { auxiliary_flags_ptr.read_unaligned() };
        unsafe { auxiliary_flags_ptr.write_unaligned(auxiliary_flags | INITIALIZED_BIT) };

        let byte_state_ptr =
            lf_checker_rt::relocated(this_ptr).wrapping_add(BYTE_STATE_OFFSET) as *mut u8;
        unsafe { byte_state_ptr.write_unaligned(u8::MAX) };
        this_ptr
    }
});
