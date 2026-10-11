// original: 0x00CE9FA0 CTaskSimpleNMSit::vf26
/// Advance the sit task's optional subtask, emit its follow-up message, and
/// continue the task through its direct helper.
///
/// The style index and variant at `this+0x28` and `this+0x2C` use the exact
/// sentinel check against `u32::MAX`. When both are present, the embedded
/// member at `this+0x64` validates the style; a nonzero handle lookup then
/// registers the task through the owner object's receiver at `+0x7B4`, sends
/// the style message, and sets the ready byte at `this+0x60`.
///
/// Regardless of that branch, the routine initializes and sends a second
/// two-parameter message, resets its buffer, and invokes the task continuation
/// helper with the owner and a value of one. The large message buffers are
/// stack-local; their addresses are skipped at calls and the buffer prefix
/// and entry count are snapshotted.
///
/// This is a thiscall routine with one stack argument and no meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_00ce9fa0(this: u32, owner: u32) -> u32 {
    unsafe {
        const STYLE: u32 = 0x28;
        const VARIANT: u32 = 0x2C;
        const READY: u32 = 0x60;
        const SIT_MEMBER: u32 = 0x64;
        const OWNER_MESSAGE_TARGET: u32 = 0x7B4;
        const MESSAGE_BYTES: usize = 0xCC4;
        const CHECK_MEMBER: u32 = 5;
        const FIND_HANDLE: u32 = 6;
        const ADD_TASK: u32 = 7;
        const CAL_INIT: u32 = 1;
        const CAL_ADD_INT: u32 = 2;
        const CAL_SEND: u32 = 3;
        const CAL_RESET: u32 = 4;
        const CAL_CONTINUE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let style = rd32(this.wrapping_add(STYLE));
        let variant = rd32(this.wrapping_add(VARIANT));
        if style != u32::MAX && variant != u32::MAX {
            let valid = lf_checker_rt::callee_thiscall!(
                CHECK_MEMBER,
                u8,
                this.wrapping_add(SIT_MEMBER),
                style
            );
            if valid != 0 {
                let handle = lf_checker_rt::callee_cdecl!(FIND_HANDLE, u32, style, variant);
                if handle != 0 {
                    let message_target = rd32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        ADD_TASK,
                        u32,
                        message_target,
                        handle,
                        0
                    );

                    let mut first_message = [0u8; MESSAGE_BYTES];
                    let first_ptr = first_message.as_mut_ptr() as u32;
                    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_INIT, u32, first_ptr);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_ADD_INT,
                        u32,
                        first_ptr,
                        lf_checker_rt::global::<u32>(0x01051CC8).read(),
                        1
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_SEND,
                        u32,
                        message_target,
                        lf_checker_rt::global::<u32>(0x01051CCC).read(),
                        first_ptr
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, first_ptr);
                    (this.wrapping_add(READY) as *mut u8).write(1);
                }
            }
        }

        let message_target = rd32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
        let mut final_message = [0u8; MESSAGE_BYTES];
        let final_ptr = final_message.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_INIT, u32, final_ptr);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_INT,
            u32,
            final_ptr,
            lf_checker_rt::global::<u32>(0x01051CC8).read(),
            1
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_INT,
            u32,
            final_ptr,
            lf_checker_rt::global::<u32>(0x01051FD4).read(),
            0
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_SEND,
            u32,
            message_target,
            lf_checker_rt::global::<u32>(0x01051FB4).read(),
            final_ptr
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, final_ptr);
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_CONTINUE, u32, this, owner, 1);
    }
    0
});
