// original: 0x00c41e50 CIntermezzoEvent::spawn_primary_cam
/// Create and initialize a primary camera for an event argument. The event's
/// camera-list pointer is at +0x114; the factory receives the list in ECX and
/// the event, zero and type 0x21 as stack parameters. Initialize the returned
/// camera through vtable slot 1, set its attachment bits at +0x13c, and return
/// the camera pointer. This stdcall method consumes its one pointer argument.
lf_checker_rt::export!(stdcall, rw_00c41e50(event: u32) -> u32 {
    const CAMERA_LIST: u32 = 0x114;
    const CREATE_CAMERA: u32 = 1;
    const CAMERA_VTABLE_INIT: u32 = 2;
    const VTABLE_INIT_SLOT: u32 = 4;
    const ATTACHMENT_FLAGS: u32 = 0x13c;
    let list = unsafe { ((event + CAMERA_LIST) as *const u32).read_unaligned() };
    let camera = lf_checker_rt::callee_thiscall!(CREATE_CAMERA, u32, list, 0x21, 0, event);
    let vtable = unsafe { (camera as *const u32).read_unaligned() };
    let init_address = unsafe { ((vtable + VTABLE_INIT_SLOT) as *const u32).read_unaligned() };
    let init: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(init_address as usize) };
    let _ = unsafe { init(camera) };
    unsafe {
        let flags = (camera + ATTACHMENT_FLAGS) as *mut u8;
        flags.write_unaligned(flags.read_unaligned() | 0x0c);
    }
    camera
});
