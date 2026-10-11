// original: 0x00b45200 CEventDraggedOutCar::CEventDraggedOutCar
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the editable base constructor, then installs its own vtable; first_reference is stored at +0x1C (4 byte(s); second_reference is stored at +0x18 (4 byte(s); first_kind is stored at +0x20 (1 byte(s); second_kind is stored at +0x21 (1 byte(s); reference-link calls use the object field address as their stack argument and the stored reference as ECX.
/// The returned EAX value is the receiver.
lf_checker_rt::export!(thiscall, rw_00b45200(this: u32, first_reference: u32, second_reference: u32, first_kind: u32, second_kind: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00eadba4;
        const FIELD_20: u32 = 0x00000020;
        const FIELD_21: u32 = 0x00000021;
        const FIELD_18: u32 = 0x00000018;
        const FIELD_1C: u32 = 0x0000001c;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        let _ = GLOBAL_EVENT_TOKEN;
        #[inline(always)] unsafe fn read_u32(address: u32) -> u32 { core::ptr::read_unaligned(address as *const u32) }
        #[inline(always)] unsafe fn write_u32(address: u32, value: u32) { core::ptr::write_unaligned(address as *mut u32, value); }
        #[inline(always)] unsafe fn write_u16(address: u32, value: u16) { core::ptr::write_unaligned(address as *mut u16, value); }
        #[inline(always)] unsafe fn write_u8(address: u32, value: u8) { core::ptr::write(address as *mut u8, value); }
        #[inline(always)] unsafe fn copy_words(destination: u32, source: u32, count: usize) {
            for index in 0..count {
                let value = read_u32(source.wrapping_add((index as u32).wrapping_mul(4)));
                write_u32(destination.wrapping_add((index as u32).wrapping_mul(4)), value);
            }
        }
    unsafe {
        let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        write_u32(this.wrapping_add(VTABLE_FIELD), lf_checker_rt::relocated(VTABLE_FILE_VA));
        write_u32(this.wrapping_add(FIELD_1C), first_reference);
        write_u32(this.wrapping_add(FIELD_18), second_reference);
        write_u8(this.wrapping_add(FIELD_20), first_kind as u8);
        write_u8(this.wrapping_add(FIELD_21), second_kind as u8);
        if read_u32(this.wrapping_add(FIELD_1C)) != 0 {
            let _link_result = lf_checker_rt::callee_thiscall!(2, u32, read_u32(this.wrapping_add(FIELD_1C)), this.wrapping_add(FIELD_1C));
        }
        if read_u32(this.wrapping_add(FIELD_18)) != 0 {
            let _link_result = lf_checker_rt::callee_thiscall!(2, u32, read_u32(this.wrapping_add(FIELD_18)), this.wrapping_add(FIELD_18));
        }
        this
    }
});
