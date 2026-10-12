// original: 0x00A86130 CComplexGangDrivebyTaskInfo::vf4

/// Write the gang-driveby task's flag and either its variable payload fields or its inline data block.
///
/// `this` is the task-info object and `stream` is the serializer context.
/// The branch follows the original field or callback result, and each
/// delegated value keeps its observed width and argument order.
lf_checker_rt::export!(thiscall, rw_00a86130(this: u32, stream: u32) -> u32 {
    unsafe {
        const VARIANT_OFFSET: u32 = 0x18;
        let _base = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let variant = unsafe { (this.wrapping_add(VARIANT_OFFSET) as *const u16).read_unaligned() };
        let has_variant = variant != 0;
        let _flag = lf_checker_rt::callee_thiscall!(2, u32, stream, u32::from(has_variant));
        if has_variant {
            let _variant_value = lf_checker_rt::callee_thiscall!(3, u32, stream, u32::from(variant));
            let _padding = lf_checker_rt::callee_thiscall!(4, u32, stream, 0u32, 0x20u32);
            let length = lf_checker_rt::callee_thiscall!(5, u32, stream);
            let _adjustment = lf_checker_rt::callee_thiscall!(4, u32, stream, 0u32, length.wrapping_sub(0x2e));
        } else {
            let inline_data = this.wrapping_add(0x20);
            let _inline = lf_checker_rt::callee_thiscall!(6, u32, stream, inline_data, 0x13u32);
        }
        let bit_31 = unsafe { (this.wrapping_add(0x31) as *const u8).read() };
        let _ = lf_checker_rt::callee_thiscall!(2, u32, stream, u32::from(bit_31));
        let bit_32 = unsafe { (this.wrapping_add(0x32) as *const u8).read() };
        lf_checker_rt::callee_thiscall!(2, u32, stream, u32::from(bit_32))
    }
});
