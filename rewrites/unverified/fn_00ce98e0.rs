// original: 0x00CE98E0 CTaskSimpleNMJumpRollFromRoadVehicle::vf26
/// Build and send the jump-roll behaviour message, then copy a helper-provided
/// four-word value into the task's vector area.
///
/// The `owner` argument supplies the message receiver at offset `+0x7B4`.
/// A 0xCC4-byte stack message buffer is initialized, receives one integer and
/// four float parameters, is sent using the task-specific message identifier,
/// and is reset. A second helper is called with two scratch pointers and two
/// zero flags; its returned four-word block is copied to `this+0x30..0x3F`.
/// The readiness byte at `this+0x40` is cleared.
///
/// Buffer and scratch pointers refer to stack locals whose addresses depend
/// on the compiler frame. The call contract skips those pointer values and
/// snapshots their contents; all message identifiers, scalar arguments,
/// helper order, vector words, object writes, and stack cleanup are compared.
///
/// This is a thiscall routine with one stack argument and no meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_00ce98e0(this: u32, owner: u32) -> u32 {
    unsafe {
        const MESSAGE_BYTES: usize = 0xCC4;
        const OWNER_MESSAGE_TARGET: u32 = 0x7B4;
        const TASK_VECTOR: u32 = 0x30;
        const TASK_READY: u32 = 0x40;
        const CAL_INIT: u32 = 1;
        const CAL_ADD_INT: u32 = 2;
        const CAL_ADD_FLOAT: u32 = 3;
        const CAL_SEND: u32 = 4;
        const CAL_RESET: u32 = 5;
        const CAL_VECTOR: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut message = [0u8; MESSAGE_BYTES];
        let message_ptr = message.as_mut_ptr() as u32;
        let message_open = lf_checker_rt::global::<u32>(0x01051CC8).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_INIT, u32, message_ptr);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_INT,
            u32,
            message_ptr,
            message_open,
            1
        );

        for (name_va, value_va) in [
            (0x01051FBC, 0x0171CDDC),
            (0x01051FC0, 0x0171CDDC),
            (0x01051FC4, 0x0171CDDC),
            (0x01051FD0, 0x0171CE10),
        ] {
            let name = lf_checker_rt::global::<u32>(name_va).read();
            let value = lf_checker_rt::global::<u32>(value_va).read();
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CAL_ADD_FLOAT,
                u32,
                message_ptr,
                name,
                value
            );
        }

        let target = rd32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
        let message_id = lf_checker_rt::global::<u32>(0x01051FB4).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_SEND,
            u32,
            target,
            message_id,
            message_ptr
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, message_ptr);

        let mut first_scratch = [0u32; 4];
        let mut second_scratch = [0u32; 4];
        let values = lf_checker_rt::callee_thiscall!(
            CAL_VECTOR,
            u32,
            owner,
            first_scratch.as_mut_ptr() as u32,
            second_scratch.as_mut_ptr() as u32,
            0,
            0
        );
        for offset in [0u32, 4, 8, 12] {
            wr32(
                this.wrapping_add(TASK_VECTOR + offset),
                rd32(values.wrapping_add(offset)),
            );
        }
        (this.wrapping_add(TASK_READY) as *mut u8).write(0);
    }
    0
});
