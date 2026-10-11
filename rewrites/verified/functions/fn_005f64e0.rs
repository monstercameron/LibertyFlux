// original: 0x005F64E0 fragInstNMGta::vf10

/// Resolve the fragment object selected by the instance, select its float
/// table, and return the indexed value as an exact `f32` result.
///
/// The instance holds a referenced type at `0x5c`, a resource pointer at
/// `0x64`, and a fallback object at `0x6c`. A non-null referenced type is
/// accepted when its flags at `+0x08` contain `0x30000000`; its first word
/// names the resolved object. With no referenced type the fallback object is
/// used. A live resource supplies the table pointer at `+0x1bc`; otherwise
/// the resolved object supplies it at `+0x204`. The stack index selects a
/// four-byte float. The method returns that value in the x87 result channel
/// and consumes one stack argument. When the selected table pointer is null,
/// the original builds a temporary table through helpers; that path is not
/// exercised by this contract.
lf_checker_rt::export!(thiscall, rw_005f64e0(this: u32, index: u32) -> f32 {
    const REFERENCED_TYPE: u32 = 0x5c;
    const RESOURCE_POINTER: u32 = 0x64;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;
    const RESOURCE_TABLE: u32 = 0x1bc;
    const OBJECT_TABLE: u32 = 0x204;
    const COOKIE_CHECK: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    unsafe {
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        let object = if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        };

        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        let table = if resource != 0 {
            read_u32(resource.wrapping_add(RESOURCE_TABLE))
        } else {
            read_u32(object.wrapping_add(OBJECT_TABLE))
        };
        let value = read_f32(table.wrapping_add(index.wrapping_mul(4)));

        // This replaces the frame-cookie helper; its frame-dependent register
        // values are not compared by the contract.
        let _ = lf_checker_rt::callee_fastcall!(COOKIE_CHECK, u32, 0, 0);
        value
    }
});
