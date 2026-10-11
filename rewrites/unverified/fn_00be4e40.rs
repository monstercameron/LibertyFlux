// original: 0x00BE4E40 task_type_dd_transition

/// Ask the child object for its type code. Values other than `0xdd` return
/// directly. For `0xdd`, obtain the type descriptor and the current property,
/// compare the descriptor lookup result with the child's value at `0x34`, and
/// if the helper reports a match and those values differ, call the transition
/// helper and set `this+0x87` to one. The return register is the last helper's
/// value on each path.
///
/// The object is thiscall with no stack arguments. Its child pointer is at
/// `+8`, the comparison value is at child `+0x34`, and the property word is at
/// `+0x3c`. Direct helper calls are represented by scripted callees.
lf_checker_rt::export!(thiscall, rw_00be4e40(this: u32) -> u32 {
    unsafe {
        const CHILD_POINTER: u32 = 0x08;
        const TYPE_SLOT: u32 = 0x0c;
        const CHILD_VALUE: u32 = 0x34;
        const PROPERTY_VALUE: u32 = 0x3c;
        const TRANSITION_FLAG: u32 = 0x87;
        const TYPE_DD: u32 = 0xdd;
        const TYPE_INFO: u32 = 0x00eb8940;
        let read_u32 = |address: u32| (address as *const u32).read_unaligned();
        let write_u8 = |address: u32, value: u8| (address as *mut u8).write(value);
        let child = read_u32(this + CHILD_POINTER);
        let vtable = read_u32(child);
        let type_method = read_u32(vtable + TYPE_SLOT);
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(type_method as usize);
        let type_code = get_type(child);
        if type_code != TYPE_DD {
            return type_code;
        }
        let type_descriptor = lf_checker_rt::relocated(TYPE_INFO);
        let info = lf_checker_rt::callee_cdecl!(2, u32, type_descriptor, 0);
        let property = read_u32(this + PROPERTY_VALUE);
        let item = lf_checker_rt::callee_cdecl!(3, u32, property);
        let matched = lf_checker_rt::callee_cdecl!(4, u32, item, info);
        if matched == 0 || read_u32(child + CHILD_VALUE) == item {
            return matched;
        }
        let result = lf_checker_rt::callee_thiscall!(5, u32, child, item, info, 1);
        write_u8(this + TRANSITION_FLAG, 1);
        result
    }
});
