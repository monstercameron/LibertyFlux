// original: 0x00CE9D60 CTaskSimpleNMRollUpAndRelax::vf26
/// Build and send the roll-up-and-relax behaviour message from the owner
/// object's message receiver and the shared task parameter table.
///
/// The task's base vtable argument is unused by the body. It initializes a
/// 0xCC4-byte stack message, adds an integer and a label, adds a random value
/// in the original multiply/add order, adds three table floats, then adds a
/// second randomised float before sending and resetting the buffer. The
/// shared parameter table at offsets `0x171CFE4`, `0x171CFE8` and `0x171CFEC`
/// is runtime-filled; the contract seeds each word with float edge values.
///
/// The buffer pointer is a stack-local address, so its register and send
/// argument are skipped while the helper arguments and the buffer prefix
/// plus entry count are compared. The runtime's float operations are pinned
/// to the original operand order for bit-exact results.
///
/// This is a thiscall routine with one stack argument and no meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_00ce9d60(this: u32, owner: u32) -> u32 {
    unsafe {
        const MESSAGE_BYTES: usize = 0xCC4;
        const OWNER_MESSAGE_TARGET: u32 = 0x7B4;
        const CAL_INIT: u32 = 1;
        const CAL_ADD_INT: u32 = 2;
        const CAL_ADD_LABEL: u32 = 3;
        const CAL_RNG: u32 = 4;
        const CAL_ADD_FLOAT: u32 = 5;
        const CAL_SEND: u32 = 6;
        const CAL_RESET: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(address: u32) -> f32 {
            f32::from_bits(unsafe { rd32(address) })
        }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let _task = this;
        let mut message = [0u8; MESSAGE_BYTES];
        let message_ptr = message.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_INIT, u32, message_ptr);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_INT,
            u32,
            message_ptr,
            lf_checker_rt::global::<u32>(0x01051CC8).read(),
            1
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_LABEL,
            u32,
            message_ptr,
            lf_checker_rt::global::<u32>(0x01051D4C).read(),
            lf_checker_rt::relocated(0x00EDB140)
        );

        let random_near = lf_checker_rt::callee_cdecl!(CAL_RNG, u32,);
        let mut near_parameter = mul(
            random_near as f32,
            rdf(lf_checker_rt::relocated(0x00FE8684)),
        );
        near_parameter = mul(near_parameter, rdf(lf_checker_rt::relocated(0x00FE8A24)));
        near_parameter = core::hint::black_box(near_parameter)
            + core::hint::black_box(rdf(lf_checker_rt::relocated(0x00FE8B08)));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_FLOAT,
            u32,
            message_ptr,
            lf_checker_rt::global::<u32>(0x01051D50).read(),
            near_parameter.to_bits()
        );

        for (name_va, value_va) in [
            (0x01051D54, 0x0171CFE4),
            (0x01051D58, 0x0171CFE8),
            (0x01051D5C, 0x0171CFEC),
        ] {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CAL_ADD_FLOAT,
                u32,
                message_ptr,
                lf_checker_rt::global::<u32>(name_va).read(),
                lf_checker_rt::global::<u32>(value_va).read()
            );
        }

        let random_far = lf_checker_rt::callee_cdecl!(CAL_RNG, u32,);
        let scaled = mul(
            random_far as f32,
            rdf(lf_checker_rt::relocated(0x00FE8684)),
        );
        let mut far_parameter = mul(
            scaled,
            rdf(lf_checker_rt::relocated(0x00FE87E8)),
        );
        far_parameter = core::hint::black_box(far_parameter)
            + core::hint::black_box(rdf(lf_checker_rt::relocated(0x00FE879C)));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_FLOAT,
            u32,
            message_ptr,
            lf_checker_rt::global::<u32>(0x01051D60).read(),
            far_parameter.to_bits()
        );

        let target = rd32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
        let message_id = lf_checker_rt::global::<u32>(0x01051D44).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_SEND,
            u32,
            target,
            message_id,
            message_ptr
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, message_ptr);
    }
    0
});
