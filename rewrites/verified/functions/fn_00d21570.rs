// original: 0x00D21570 bake_fx_raycast_sample
/// Routes the requested sample mode to one of three helpers. Modes one and
/// two first release the existing sample array, then forward the object,
/// caller context and fixed float-bit parameters. Mode three lets a helper
/// fill a four-word local record and copies those words to the object's start.
/// Other modes return without calling a helper. The routine is stdcall with
/// three stack words; helper data is compared at the call boundary.
lf_checker_rt::export!(stdcall, rw_00d21570(object: u32, mode: u32, context: u32) -> u32 {
    const MODE_RAYCAST: u32 = 1;
    const MODE_CAPTURE: u32 = 2;
    const MODE_BAKED: u32 = 3;
    const POSITION_X_BITS: u32 = 0x4080_0000;
    const POSITION_Y_BITS: u32 = 0x40c0_0000;
    const RAYCAST_RADIUS_BITS: u32 = 0xbf80_0000;
    const CAPTURE_RADIUS_BITS: u32 = 0x4020_0000;
    const SAMPLE_TABLE_FILE_VA: u32 = 0x00c1_9080;
    const SAMPLE_END_FILE_VA: u32 = 0x0062_e7a0;

    if mode == MODE_RAYCAST {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, object);
        let _ = lf_checker_rt::callee_cdecl!(2, u32, object, context, POSITION_X_BITS, POSITION_Y_BITS, RAYCAST_RADIUS_BITS);
    } else if mode == MODE_CAPTURE {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, object);
        let _ = lf_checker_rt::callee_cdecl!(3, u32, object, context, CAPTURE_RADIUS_BITS, RAYCAST_RADIUS_BITS);
    } else if mode == MODE_BAKED {
        let mut local_words = [0u32; 4];
        let local_context = local_words.as_mut_ptr() as u32;
        let sample_table = lf_checker_rt::relocated(SAMPLE_TABLE_FILE_VA);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, local_context, 0u32, sample_table, 0u32, 0u32);
        let local_words_ptr = local_words.as_ptr();
        let words = unsafe { [
            core::ptr::read_volatile(local_words_ptr),
            core::ptr::read_volatile(local_words_ptr.add(1)),
            core::ptr::read_volatile(local_words_ptr.add(2)),
            lf_checker_rt::relocated(SAMPLE_END_FILE_VA),
        ] };
        unsafe {
            (object as *mut u32).write_unaligned(words[0]);
            (object.wrapping_add(4) as *mut u32).write_unaligned(words[1]);
            (object.wrapping_add(8) as *mut u32).write_unaligned(words[2]);
            (object.wrapping_add(12) as *mut u32).write_unaligned(words[3]);
        }
    }
    0
});
