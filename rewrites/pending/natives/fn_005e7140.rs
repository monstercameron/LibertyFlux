// original: 0x005e7140 CELL_CAM_SET_COLOUR_BRIGHTNESS
/// Script native `CELL_CAM_SET_COLOUR_BRIGHTNESS` (hash 0x4ECB189E).
///
/// Copies four float script arguments (colour and brightness components) as
/// raw bits into four engine globals, then resolves the phone-camera object:
/// the first engine query returns a camera id (or -1 when there is none) and
/// the second resolves that id to an object pointer (or null). When an
/// object is found the four components are also stored into its fields at
/// offset 0xd60. No return slot is written.
export!(cdecl, rw_005e7140(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let v0 = *args;
        let v1 = *args.add(1);
        let v2 = *args.add(2);
        let v3 = *args.add(3);
        // Same write order as the original.
        *global::<u32>(0x106C318) = v0;
        *global::<u32>(0x106C328) = v1;
        *global::<u32>(0x106C314) = v2;
        *global::<u32>(0x106C320) = v3;
        // 0x161547C is a relocated engine pointer (HIGHLOW reloc on the
        // immediate), not a raw constant.
        let engine = relocated(0x161547C);
        let cam_id = callee_thiscall!(1, u32, engine);
        if cam_id == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let obj = callee_thiscall!(2, u32, engine, cam_id);
        if obj == 0 {
            return 0;
        }
        *((obj + 0xD60) as *mut u32) = v0;
        *((obj + 0xD64) as *mut u32) = v1;
        *((obj + 0xD68) as *mut u32) = v2;
        *((obj + 0xD6C) as *mut u32) = v3;
        obj
    }
});
