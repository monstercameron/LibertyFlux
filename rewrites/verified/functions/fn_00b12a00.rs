// original: 0x00B12A00 audPlaceableTracker::audPlaceableTracker_3
/// Construct an audio placeable tracker in an existing object. The object
/// arrives in ECX; the three stack words are a model index followed by two
/// creation flags whose low bytes control physics and drawable setup. The
/// function initializes its base and embedded trackers, makes the matching
/// helper calls in order, and returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_00b12a00(object: u32, model_index: u32, drawable_flags: u32, physics_flags: u32) -> u32 {
    const OBJECT_VTABLE_FILE_VA: u32 = 0x00EAB0DC;
    const TRACKER_VTABLE_FILE_VA: u32 = 0x00E985D8;
    const TRACKER_MEMBER: u32 = 0x264;
    const TRACKER_NULL_FIELD: u32 = 0x26C;
    const AUDIO_ENTITY_MEMBER: u32 = 0x270;
    const ATTACH_MODEL_AUDIO: u32 = u32::MAX;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, object);
        (object as *mut u32).write_volatile(lf_checker_rt::relocated(OBJECT_VTABLE_FILE_VA));

        let tracker = object.wrapping_add(TRACKER_MEMBER);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, tracker);
        (tracker as *mut u32).write_volatile(lf_checker_rt::relocated(TRACKER_VTABLE_FILE_VA));
        (object.wrapping_add(TRACKER_NULL_FIELD) as *mut u32).write_volatile(0);

        let audio_entity = object.wrapping_add(AUDIO_ENTITY_MEMBER);
        let _ = lf_checker_rt::callee_thiscall!(3, u32, audio_entity);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, object);
        let _ = lf_checker_rt::callee_thiscall!(5, u32, object, model_index);
        if physics_flags as u8 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(6, u32, object);
        }
        if drawable_flags as u8 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(7, u32, object);
        }
        let _ = lf_checker_rt::callee_thiscall!(8, u32, object, ATTACH_MODEL_AUDIO);
        object
    }
});
