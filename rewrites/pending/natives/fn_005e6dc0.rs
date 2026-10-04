// original: 0x005e6dc0 SET_MOBILE_PHONE_ROTATION
/// Script native `SET_MOBILE_PHONE_ROTATION` (hash 0x7E7E4879).
///
/// Stores three script arguments (rotation angles, as float bit-patterns)
/// directly into three consecutive engine globals. Makes no engine call.
/// The handler exits with the argument-array pointer in EAX, which this
/// rewrite reproduces as its return value.
export!(cdecl, rw_005e6dc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        *global::<u32>(0x019D3B30) = *args;
        *global::<u32>(0x019D3B34) = *args.add(1);
        *global::<u32>(0x019D3B38) = *args.add(2);
        args as u32
    }
});
