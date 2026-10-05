// original: 0x00a9cbf0 stream_register_and_link (proposed)

/// Register `obj` with the three registrar routines and link it when pending.
///
/// `obj` is offered to the validator (object in ECX), then appended to the
/// table at `this + 0x0c`, then registered with the shared registrar at
/// file address 0x01305d30 (which runs the neighbouring `+0x8d830` thunk).
/// When the pending word at `obj + 0x18` is positive the linker at file
/// address 0x013baba0 runs with (`[this + 0x68]`, `obj`) and the pending
/// word is cleared. The result is the registrar's answer, or the linker's
/// when it ran.
///
/// Original: 0x00a9cbf0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a9cbf0(this: u32, obj: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x0c;
        const PENDING_OFF: u32 = 0x18;
        const LINK_OFF: u32 = 0x68;
        const REGISTRAR: u32 = 0x01305d30;
        const LINKER: u32 = 0x013baba0;
        const VALIDATE: u32 = 1;
        const APPEND: u32 = 2;
        const REGISTER: u32 = 3;
        const LINK: u32 = 4;
        lf_checker_rt::callee_thiscall!(VALIDATE, u32, obj);
        lf_checker_rt::callee_thiscall!(APPEND, u32, this.wrapping_add(TABLE_OFF), obj);
        let answer: u32 = lf_checker_rt::callee_thiscall!(
            REGISTER,
            u32,
            lf_checker_rt::relocated(REGISTRAR),
            obj
        );
        if ((obj + PENDING_OFF) as *const u32).read_unaligned() as i32 > 0 {
            let link = ((this + LINK_OFF) as *const u32).read_unaligned();
            let done: u32 = lf_checker_rt::callee_thiscall!(
                LINK,
                u32,
                lf_checker_rt::relocated(LINKER),
                link,
                obj
            );
            ((obj + PENDING_OFF) as *mut u32).write_unaligned(0);
            return done;
        }
        answer
    }
});
