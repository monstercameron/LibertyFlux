// original: 0x00cd56e0 euphoria_static_thunk
/// Static entry thunk: binds ECX to the module-global object and tail-jumps
/// to the shared routine.
///
/// The tag immediate carries a HIGHLOW relocation, so the runtime value is
/// image-base-relative and must be derived with `relocated`.
export!(cdecl, rw_cd56e0() -> u32 {
    callee_thiscall!(1, u32, relocated(0x171C968))
});
