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

const RNG: u32 = 1;
const RNG_SCALE_VA: u32 = 0x00FE8680;
const RNG_TABLE_SCALE_VA: u32 = 0x00FE8DC8;
const RUNNING_BASE_VA: u32 = 0x011735B4;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

#[inline(always)]
fn multiply(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn cvttss2si(value: f32) -> i32 {
    if value.is_nan() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
        i32::MIN
    } else {
        value as i32
    }
}

unsafe fn build_relax_message(owner: u32, mutant: bool) {
    const MESSAGE_BYTES: usize = 0xCC4;
    const OWNER_MESSAGE_TARGET: u32 = 0x7B4;
    const CAL_INIT: u32 = 1;
    const CAL_ADD_INT: u32 = 2;
    const CAL_ADD_LABEL: u32 = 3;
    const CAL_RNG: u32 = 4;
    const CAL_ADD_FLOAT: u32 = 5;
    const CAL_SEND: u32 = 6;
    const CAL_RESET: u32 = 7;

    unsafe {
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
        let mut first_parameter = multiply(
            random_near as f32,
            read_f32(lf_checker_rt::relocated(0x00FE8684)),
        );
        first_parameter = multiply(first_parameter, read_f32(lf_checker_rt::relocated(0x00FE8A24)));
        first_parameter = core::hint::black_box(first_parameter)
            + core::hint::black_box(read_f32(lf_checker_rt::relocated(0x00FE8B08)));
        let first_name = lf_checker_rt::global::<u32>(0x01051D50).read();
        let mut first_bits = first_parameter.to_bits();
        if mutant {
            first_bits ^= 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_FLOAT,
            u32,
            message_ptr,
            first_name,
            first_bits
        );

        for (name_va, value_va) in [
            (0x01051D54, 0x0171CFE4),
            (0x01051D58, 0x0171CFE8),
            (0x01051D5C, 0x0171CFEC),
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

        let random_far = lf_checker_rt::callee_cdecl!(CAL_RNG, u32,);
        let scaled = multiply(
            random_far as f32,
            read_f32(lf_checker_rt::relocated(0x00FE8684)),
        );
        let mut last_parameter = multiply(
            scaled,
            read_f32(lf_checker_rt::relocated(0x00FE87E8)),
        );
        last_parameter = core::hint::black_box(last_parameter)
            + core::hint::black_box(read_f32(lf_checker_rt::relocated(0x00FE879C)));
        let last_name = lf_checker_rt::global::<u32>(0x01051D60).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_FLOAT,
            u32,
            message_ptr,
            last_name,
            last_parameter.to_bits()
        );

        let target = read_u32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
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
}

lf_checker_rt::export!(thiscall, rw_00ce9d60(this: u32, owner: u32) -> u32 {
    let _ = this;
    unsafe { build_relax_message(owner, false) }
    0
});
