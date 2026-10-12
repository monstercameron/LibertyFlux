// original: 0x0098b240 audRadioEmitter::~audRadioEmitter__deleting
/// Installs the radio-emitter vtable and optionally releases the object.
///
/// The thiscall method always writes the relocated deleting-destructor vtable
/// address and returns the receiver. If bit zero of its one stack argument
/// is set, it calls the intercepted cdecl release function with the receiver.
const DELETING_VTABLE_VA: u32 = 0x00E8_E23C;
const RELEASE_CALLEE: u32 = 1;

export!(thiscall, rw_0098b240(this_: u32, delete_flags: u32) -> u32 {
    unsafe {
        (this_ as *mut u32).write(lf_checker_rt::relocated(DELETING_VTABLE_VA));
    }
    if delete_flags & 1 != 0 {
        let _release_result = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, this_);
    }
    this_
});
