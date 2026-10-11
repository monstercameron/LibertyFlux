// original: 0x00c41e80 CIntermezzoEvent::release_cam
/// Release an event's attached camera when present, then clear the camera
/// pointer at byte offset 4. The original is a thiscall method with no
/// stack parameters. Its release helper is thiscall with one zero stack
/// argument; the helper's return value is ignored.
lf_checker_rt::export!(thiscall, rw_00c41e80(this: u32) -> u32 {
    const CAMERA: u32 = 4;
    const RELEASE: u32 = 1;

    unsafe {
        let camera = ((this + CAMERA) as *const u32).read_unaligned();
        if camera != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, camera, 0);
            ((this + CAMERA) as *mut u32).write_unaligned(0);
        }
    }
    0
});
