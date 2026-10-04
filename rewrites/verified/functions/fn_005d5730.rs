// original: 0x005d5730 anim_register_three_sets
/// Register three animation association sets (slots `0x8c`-`0x8e`) on this
/// collection after running the base initialiser. Each registration passes
/// the shared table pointer read from its global plus the set's own table
/// pointers. Returns the last registration answer.
export!(thiscall, rw_005d5730(this_ptr: u32) -> u32 {
    let _: u32 = callee_thiscall!(0, u32, this_ptr);
    let shared = unsafe { global::<u32>(0x0104B30C).read() };
    let libs = relocated(0x0104B208);
    let names = relocated(0x0104B310);
    let _: u32 = callee_thiscall!(
        1, u32, this_ptr, 0x8C,
        relocated(0x00F90608), relocated(0x00F905F8),
        shared, libs, names, 1, 2, 4, 0
    );
    let _: u32 = callee_thiscall!(
        1, u32, this_ptr, 0x8D,
        relocated(0x00F90628), relocated(0x00F90618),
        shared, libs, names, 1, 2, 4, 0
    );
    callee_thiscall!(
        1, u32, this_ptr, 0x8E,
        relocated(0x00F905E8), relocated(0x00F905D8),
        shared, libs, names, 1, 2, 4, 0
    )
});
