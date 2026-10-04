// original: 0x008949d0 init_with_global_flags
/// Forward stored global flags and fixed options to the object initializer.
///
/// Reads one global flags word and passes it with two constant option words
/// to the initializer, returning the initializer's result.
crate::rt::export!(thiscall, rs17_008949d0(this: u32) -> u32 {
    let flags = unsafe { *crate::rt::global::<u32>(0x0115DE9C) };
    crate::callee_thiscall!(4, u32, this, flags, 1, 0x0F)
});
