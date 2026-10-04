// original: 0x00b86bd0 GET_CAM_ROT
/// Native handler `GET_CAM_ROT`: forwards four script words (output pointers) to the engine and returns nothing itself.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_get_cam_rot(ctx: u32) {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        lf_rn24_rt::callee_cdecl!(1, (), unsafe { args.read() }, unsafe { args.add(1).read() }, unsafe { args.add(2).read() }, unsafe { args.add(3).read() });
}
