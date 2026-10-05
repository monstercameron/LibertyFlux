// original: 0x00A00640 CLEAR_ROOM_FOR_OBJECT
//
// Script native handler: forwards the script object handle to one engine
// function (cdecl/1, caller cleans with `(an instruction of the original)`). No return value stored.
export!(cdecl, rw_00a00640(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args)
    }
});
