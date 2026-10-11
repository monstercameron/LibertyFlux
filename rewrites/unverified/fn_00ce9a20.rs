// original: 0x00CE9A20 CTaskSimpleNMOnFire::vf26
/// Send the on-fire task's behaviour message and advance its table-backed
/// state.
///
/// The style index at `this+0x4C` selects one 0x68-byte row from a runtime
/// parameter table. Nine words from that row are sent as float message
/// fields. The message is sent through the owner receiver at `+0x7B4`, the
/// flag at `this+0x28` is set, and a nonzero byte at `this+0x50` also stores
/// the row's time value plus the shared running base at `this+0x44`.
/// Finally, the task's method at vtable slot `0x6C` is called with the owner.
///
/// The two 0xCC4-byte message buffers are local. Their addresses are skipped
/// at send calls and their content and entry counts are snapshotted. The
/// table values are scripted as individual global words; the vtable method
/// is planted in the fabricated task object. This is a thiscall routine with
/// one stack argument and no meaningful return value.

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

unsafe fn build_on_fire_message(this: u32, owner: u32, mutant: bool) {
    const MESSAGE_BYTES: usize = 0xCC4;
    const TASK_STYLE: u32 = 0x4C;
    const TASK_FLAGS: u32 = 0x28;
    const TASK_STYLE_OFFSET: u32 = 0x44;
    const TASK_USE_TABLE_VALUE: u32 = 0x50;
    const OWNER_MESSAGE_TARGET: u32 = 0x7B4;
    const TABLE_BASE: u32 = 0x0171CA44;
    const TABLE_STRIDE: u32 = 0x68;
    const CAL_FORMAT: u32 = 1;
    const CAL_INIT: u32 = 2;
    const CAL_ADD_INT: u32 = 3;
    const CAL_SEND: u32 = 4;
    const CAL_ADD_FLOAT: u32 = 5;
    const CAL_ADD_LABEL: u32 = 6;
    const CAL_RESET: u32 = 7;

    let style = read_u32(this.wrapping_add(TASK_STYLE));
    let table_row = TABLE_BASE.wrapping_add(style.wrapping_mul(TABLE_STRIDE));
    let mut first_message = [0u8; MESSAGE_BYTES];
    let mut message = [0u8; MESSAGE_BYTES];
    let first_ptr = first_message.as_mut_ptr() as u32;
    let message_ptr = message.as_mut_ptr() as u32;

    let table_name = lf_checker_rt::global::<u32>(
        0x01051C40u32.wrapping_add(style.wrapping_mul(4)),
    )
    .read_unaligned();
    let _: u32 = lf_checker_rt::callee_cdecl!(
        CAL_FORMAT,
        u32,
        lf_checker_rt::relocated(0x00EDB304),
        style,
        table_name
    );
    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_INIT, u32, first_ptr);
    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_ADD_INT,
        u32,
        first_ptr,
        lf_checker_rt::global::<u32>(0x01051CC8).read(),
        1
    );
    let first_target = read_u32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_SEND,
        u32,
        first_target,
        lf_checker_rt::global::<u32>(0x01051DFC).read(),
        first_ptr
    );

    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_INIT, u32, message_ptr);
    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_ADD_INT,
        u32,
        message_ptr,
        lf_checker_rt::global::<u32>(0x01051CC8).read(),
        1
    );

    for (value_offset, name_va) in [
        (0x00, 0x01051D6C),
        (0x04, 0x01051D88),
        (0x08, 0x01051D7C),
        (0x0C, 0x01051D94),
        (0x10, 0x01051DA0),
        (0x14, 0x01051D74),
        (0x18, 0x01051D8C),
        (0x1C, 0x01051D80),
        (0x20, 0x01051D98),
    ] {
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_ADD_FLOAT,
            u32,
            message_ptr,
            lf_checker_rt::global::<u32>(name_va).read(),
            lf_checker_rt::global::<u32>(table_row.wrapping_add(value_offset)).read_unaligned()
        );
    }

    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_ADD_LABEL,
        u32,
        message_ptr,
        lf_checker_rt::global::<u32>(0x01051DA4).read(),
        lf_checker_rt::relocated(0x00EDB348)
    );
    let message_target = read_u32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_SEND,
        u32,
        message_target,
        lf_checker_rt::global::<u32>(0x01051D64).read(),
        message_ptr
    );

    let old_flags = read_u32(this.wrapping_add(TASK_FLAGS));
    let new_flags = if mutant { old_flags & !1 } else { old_flags | 1 };
    write_u32(this.wrapping_add(TASK_FLAGS), new_flags);
    if (this.wrapping_add(TASK_USE_TABLE_VALUE) as *const u8).read() != 0 {
        let style_time = lf_checker_rt::global::<u32>(
            0x0171CAA0u32.wrapping_add(style.wrapping_mul(TABLE_STRIDE)),
        )
        .read_unaligned();
        let running_time = lf_checker_rt::global::<u32>(RUNNING_BASE_VA).read();
        write_u32(
            this.wrapping_add(TASK_STYLE_OFFSET),
            style_time.wrapping_add(running_time),
        );
    }

    let vtable = read_u32(this);
    let method_address = read_u32(vtable.wrapping_add(0x6C));
    let method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
        core::mem::transmute(method_address as usize);
    let _ = method(this, owner);

    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, message_ptr);
    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, first_ptr);
}

lf_checker_rt::export!(thiscall, rw_00ce9a20(this: u32, owner: u32) -> u32 {
    unsafe { build_on_fire_message(this, owner, false) }
    0
});
