// original: 0x00A2FDE0 CEntity::vf0

/// Runs the base entity destructor and conditionally sends a destruction message.
///
/// This deleting-destructor wrapper receives the entity in ECX and a flags word
/// on the stack. It always calls the base destructor with the entity as its
/// this pointer. When bit zero of the flags word is set, it then calls the
/// message handler with the entity as its sole cdecl argument. It returns the
/// entity pointer in EAX and removes the four-byte flags argument.
lf_checker_rt::export!(thiscall, rw_00a2fde0(receiver: u32, flags: u32) -> u32 {
    const BASE_DESTRUCTOR: u32 = 1;
    const SEND_DESTRUCTION_MESSAGE: u32 = 2;

    let _base_result =
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, receiver);
    if flags & 1 != 0 {
        let _message_result =
            lf_checker_rt::callee_cdecl!(SEND_DESTRUCTION_MESSAGE, u32, receiver);
    }
    receiver
});
