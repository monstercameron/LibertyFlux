// original: 0x00b45250 CEventEditableResponse::CEventEditableResponse
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the event base constructor, then installs its own vtable; +0x0C receives constant 0x1; +0x10 receives constant 0xc8; +0x14 receives constant 0xffffffff.
/// The returned EAX value is the receiver.
lf_checker_rt::export!(thiscall, rw_00b45250(this: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00ead874;
        const FIELD_10: u32 = 0x00000010;
        const FIELD_0C: u32 = 0x0000000c;
        const FIELD_14: u32 = 0x00000014;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        const VALUE_00000001: u32 = 0x00000001;
        const VALUE_000000C8: u32 = 0x000000c8;
        const VALUE_FFFFFFFF: u32 = 0xffffffff;
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
        write_u8(this.wrapping_add(FIELD_0C), VALUE_00000001 as u8);
        write_u32(this.wrapping_add(FIELD_10), VALUE_000000C8 as u32);
        write_u32(this.wrapping_add(FIELD_14), VALUE_FFFFFFFF as u32);
        this
    }
});
