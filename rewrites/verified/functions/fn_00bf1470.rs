// original: 0x00bf1470 mode_gated_notify_teardown
/// Mode-gated sub-object notification plus flag-gated teardown calls.
///
/// Unless the global mode word equals 2, clears the byte at `obj+0x80+0x3c`
/// and invokes slot 2 of the function table at `obj+0x80` with
/// (`obj+0x80`, 0, `this+0x18`). Then, unless bit 27 of `obj+0x24` is set,
/// issues the two teardown calls (callees 2 and 3) with (`obj`, 0). Returns
/// the second teardown answer on the teardown path, else `obj+0x24 >> 27`.
export!(thiscall, rw_bf1470(this_obj: u32, obj: u32, _unused: u32) -> u32 {
    unsafe {
        let mode = *global::<u32>(0x011f70d4);
        if mode != 2 {
            let sub = obj + 0x80;
            let vtable = *(sub as *const u32);
            *((sub + 0x3c) as *mut u8) = 0;
            let extra = *((this_obj + 0x18) as *const u32);
            let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(*((vtable + 8) as *const u32) as usize);
            slot(sub, 0, extra);
        }
        let shifted = *((obj + 0x24) as *const u32) >> 27;
        if shifted & 1 == 0 {
            callee_cdecl!(2, u32, obj, 0);
            callee_cdecl!(3, u32, obj, 0)
        } else {
            shifted
        }
    }
});
