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
lf_checker_rt::export!(thiscall, rw_00ce9a20(this: u32, owner: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

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

    let style = rd32(this.wrapping_add(TASK_STYLE));
    let table_row = TABLE_BASE.wrapping_add(style.wrapping_mul(TABLE_STRIDE));
    let mut message = [0u8; MESSAGE_BYTES];
    let mut cleanup_message = [0u8; MESSAGE_BYTES];
    let message_ptr = message.as_mut_ptr() as u32;
    let cleanup_ptr = cleanup_message.as_mut_ptr() as u32;

    let table_name = rd32(0x01051C40u32.wrapping_add(style.wrapping_mul(4)));
    let _: u32 = lf_checker_rt::callee_cdecl!(
        CAL_FORMAT,
        u32,
        lf_checker_rt::relocated(0x00EDB304),
        style,
        table_name
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
            rd32(table_row.wrapping_add(value_offset))
        );
    }

    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_ADD_LABEL,
        u32,
        message_ptr,
        lf_checker_rt::global::<u32>(0x01051DA4).read(),
        lf_checker_rt::relocated(0x00EDB348)
    );
    let message_target = rd32(owner.wrapping_add(OWNER_MESSAGE_TARGET));
    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_SEND,
        u32,
        message_target,
        lf_checker_rt::global::<u32>(0x01051D64).read(),
        message_ptr
    );

    let old_flags = rd32(this.wrapping_add(TASK_FLAGS));
    let new_flags = old_flags | 1;
    wr32(this.wrapping_add(TASK_FLAGS), new_flags);
    if rd8(this.wrapping_add(TASK_USE_TABLE_VALUE)) != 0 {
        let style_time = rd32(0x0171CAA0u32.wrapping_add(style.wrapping_mul(TABLE_STRIDE)));
        let running_time = lf_checker_rt::global::<u32>(RUNNING_BASE_VA).read();
        wr32(
            this.wrapping_add(TASK_STYLE_OFFSET),
            style_time.wrapping_add(running_time),
        );
    }

    let vtable = rd32(this);
    let method_address = rd32(vtable.wrapping_add(0x6C));
    let method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
        core::mem::transmute(method_address as usize);
    let _ = method(this, owner);

    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, cleanup_ptr);
    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, message_ptr);

    }
    0
});
