// original: 0x005E1DB0 init_html_render_state
/// Initializes the receiver's compact HTML render-state record and returns it.
/// If TLS slot zero contains a state object, its first word is passed as the
/// resolver's this pointer and the receiver's field at byte offset `0x24` is
/// passed as an output slot. A successful resolver result and nonzero output
/// invoke the state-advance helper; the field is then reset to zero in every
/// path. The remaining fields are fixed defaults, except byte-offset `0x4C`,
/// which is two when the first global mode byte is `0x6A` or the second is
/// nonzero, and zero otherwise. The method takes only its receiver in ECX.
/// Float defaults are stored by their documented IEEE-754 bit patterns.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_init_html_render_state(this: *mut u8) -> *mut u8 {
    const FIELD_RESOLVER_OUTPUT: usize = 0x24;
    const FIELD_MODE: usize = 0x4C;
    const GLOBAL_MODE_FIRST_VA: u32 = 0x0116_C250;
    const GLOBAL_MODE_SECOND_VA: u32 = 0x0116_C253;
    const WORD_NEGATIVE_ONE: u32 = 0xFFFF_FFFF;
    const FLOAT_NEGATIVE_ONE: u32 = 0xBF80_0000;
    const FLOAT_1_1: u32 = 0x3F8C_CCCD;
    const FLOAT_ONE: u32 = 0x3F80_0000;

    #[inline(always)]
    fn put_word(base: *mut u8, offset: usize, value: u32) {
        unsafe { core::ptr::write_unaligned(base.add(offset).cast::<u32>(), value) }
    }
    #[inline(always)]
    fn put_byte(base: *mut u8, offset: usize, value: u8) {
        unsafe { core::ptr::write_unaligned(base.add(offset), value) }
    }

    let tls = lf_checker_rt::tls_slot(0).cast::<u8>();
    let resolver_state = unsafe {
        core::ptr::read_unaligned(tls.add(4).cast::<u32>())
    };
    let output_ptr = unsafe { this.add(FIELD_RESOLVER_OUTPUT) };

    if resolver_state != 0 {
        let resolver_this = unsafe {
            core::ptr::read_unaligned((resolver_state as usize as *const u32).cast::<u32>())
        };
        let resolver_status = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            resolver_this,
            output_ptr as usize as u32
        );
        if resolver_status != WORD_NEGATIVE_ONE {
            let output_value = unsafe { core::ptr::read_unaligned(output_ptr.cast::<u32>()) };
            if output_value != 0 {
                let increment = lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    resolver_state,
                    output_value
                );
                unsafe {
                    core::ptr::write_unaligned(
                        output_ptr.cast::<u32>(),
                        output_value.wrapping_add(increment),
                    );
                }
            }
        }
    }

    put_word(this, 0x00, 0x12);
    put_word(this, 0x04, FLOAT_NEGATIVE_ONE);
    put_word(this, 0x08, FLOAT_NEGATIVE_ONE);
    put_word(this, 0x0C, FLOAT_NEGATIVE_ONE);
    put_word(this, 0x10, FLOAT_NEGATIVE_ONE);
    put_word(this, 0x18, 0);
    put_word(this, 0x1C, 0);
    put_word(this, 0x20, WORD_NEGATIVE_ONE);
    put_word(this, FIELD_RESOLVER_OUTPUT, 0);
    put_word(this, 0x28, 0);
    put_word(this, 0x2C, 0);
    put_word(this, 0x30, 0x19);
    put_word(this, 0x34, 0xFF00_0000);
    put_word(this, 0x38, WORD_NEGATIVE_ONE);
    put_word(this, 0x3C, WORD_NEGATIVE_ONE);
    put_word(this, 0x40, 0x13);
    put_word(this, 0x44, 0);
    put_word(this, 0x48, 0x0C);

    let mode_first = unsafe {
        core::ptr::read_unaligned(lf_checker_rt::global::<u8>(GLOBAL_MODE_FIRST_VA))
    };
    let mode_second = unsafe {
        core::ptr::read_unaligned(lf_checker_rt::global::<u8>(GLOBAL_MODE_SECOND_VA))
    };
    put_word(
        this,
        FIELD_MODE,
        if mode_first == 0x6A || mode_second != 0 { 2 } else { 0 },
    );

    put_word(this, 0x50, 0);
    put_word(this, 0x54, FLOAT_1_1);
    put_word(this, 0x58, 0xFF00_0000);
    put_word(this, 0x5C, 0x13);
    put_word(this, 0x60, 0);
    put_word(this, 0x64, 0xFF00_0000);
    put_word(this, 0x68, 0x13);
    put_word(this, 0x6C, 0);
    put_word(this, 0x70, 0xFF00_0000);
    put_word(this, 0x74, 0x13);
    put_word(this, 0x78, 0);
    put_word(this, 0x7C, 0xFF00_0000);
    put_word(this, 0x80, 0x13);
    put_word(this, 0x84, 0);
    put_word(this, 0x88, 0);
    put_word(this, 0x8C, 0);
    put_word(this, 0x90, 0);
    put_word(this, 0x94, 0);
    put_word(this, 0x98, 0);
    put_word(this, 0x9C, 0);
    put_word(this, 0xA0, 0);
    put_word(this, 0xA4, 0);
    put_byte(this, 0xB8, 0);
    put_byte(this, 0xB9, 0);
    put_word(this, 0xA8, FLOAT_ONE);
    put_word(this, 0xAC, FLOAT_ONE);
    put_word(this, 0xB0, 1);
    put_word(this, 0xB4, 1);
    put_word(this, 0xBC, 0xFF00_00FF);
    put_word(this, 0xC0, 0x13);

    this
}
