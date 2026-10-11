// original: 0x00c452a0 CCamIntermezzo::stop_active_event
/// Stop the attached event camera when present, then clear the active-camera
/// pointer and attachment flag. The camera's slot at vtable byte offset 8 is
/// called with the camera as ECX; the linked-camera notification helper then
/// receives this object. The native method has no defined return value.
lf_checker_rt::export!(thiscall, rw_00c452a0(this: u32) -> u32 {
    const ACTIVE_CAMERA: u32 = 0x144;
    const ATTACHMENT_FLAGS: u32 = 0x13c;
    const STOP_CAMERA: u32 = 1;
    const NOTIFY_LINKED: u32 = 2;
    let camera = unsafe { ((this + ACTIVE_CAMERA) as *const u32).read_unaligned() };
    if camera != 0 {
        lf_checker_rt::callee_thiscall!(STOP_CAMERA, u32, camera);
        unsafe { ((this + ACTIVE_CAMERA) as *mut u32).write_unaligned(0); }
        lf_checker_rt::callee_thiscall!(NOTIFY_LINKED, u32, this);
        unsafe {
            let flag = (this + ATTACHMENT_FLAGS) as *mut u8;
            flag.write_unaligned(flag.read_unaligned() & !8);
        }
    }
    0
});
