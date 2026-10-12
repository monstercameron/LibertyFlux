// original: 0x0098b260 audVehicleAudioEntity::~audVehicleAudioEntity__deleting
/// Runs the base destructor and optionally releases the vehicle audio entity.
///
/// The thiscall wrapper always calls its base destructor with the receiver.
/// If bit zero of its stack argument is set, it then calls the intercepted
/// cdecl release routine with that receiver. It returns the receiver and
/// cleans up the one stack argument.
const BASE_DESTRUCTOR: u32 = 1;
const RELEASE_CALLEE: u32 = 2;

export!(thiscall, rw_0098b260(this_: u32, delete_flags: u32) -> u32 {
    let _base_result = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this_);
    if delete_flags & 1 != 0 {
        let _release_result = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, this_);
    }
    this_
});
