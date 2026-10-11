// original: 0x00C41E80 CIntermezzoEvent::release_cam
/// Release the event's attached camera when non-null, then clear the pointer
/// at byte offset 4. The release helper receives that pointer in ECX and one
/// zero stack argument. The method itself is thiscall with no stack arguments.
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
