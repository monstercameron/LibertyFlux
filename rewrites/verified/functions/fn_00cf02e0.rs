// original: 0x00CF02E0 vehicle_set_mode_fields
//! Toggles the object's mode bit from the low byte of the first stack
//! argument. If the second argument is nonzero, the method writes either the
//! mode speed and limit or zeroes both fields, then optionally configures the
//! existing subobject at `0x0F88` when the signed state at `0x0F8C` is
//! positive.
//!
//! It calls the resource helper with selector `9`; a nonzero answer causes a
//! second call and a final helper call with the current speed bits as a stack
//! word. The contract checks heap changes, call order and arguments, and stack
//! adjustment. EAX is not checked because the early path leaves incoming
//! register bits; stack scratch comparison is disabled because the temporary
//! float word is observed in the final helper's logged argument. The function
//! uses thiscall with two stack arguments.
const FLAG_FIELD: usize = 0x1310;
const ENABLED_FIELD: usize = 0x0F8C;
const INITIALIZED_FIELD: usize = 0x0F88;
const SPEED_FIELD: usize = 0x1088;
const LIMIT_FIELD: usize = 0x1404;
const MODE_SPEED_BITS: u32 = 0x3E99_999A;
const MODE_LIMIT_BITS: u32 = 0xBE86_0AA6;
const RESOURCE_KEY: u32 = 9;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00cf02e0(
    this: *mut u8,
    control_word: u32,
    enable_word: u32,
) -> u32 {
    let control = control_word as u8;
    let flags = unsafe { this.add(FLAG_FIELD).read() };
    let changed_bit = (control.wrapping_shl(4) ^ flags) & 0x10;
    unsafe { this.add(FLAG_FIELD).write(flags ^ changed_bit) };

    if enable_word as u8 == 0 {
        return 0;
    }

    if control != 0 {
        unsafe {
            core::ptr::write_unaligned(this.add(SPEED_FIELD).cast::<u32>(), MODE_SPEED_BITS);
            core::ptr::write_unaligned(this.add(LIMIT_FIELD).cast::<u32>(), MODE_LIMIT_BITS);
        }
        let enabled = unsafe {
            core::ptr::read_unaligned(this.add(ENABLED_FIELD).cast::<i32>())
        };
        let initialized = unsafe {
            core::ptr::read_unaligned(this.add(INITIALIZED_FIELD).cast::<*mut u8>())
        };
        if enabled > 0 && !initialized.is_null() {
            let configure: unsafe extern "thiscall" fn(*mut u8) -> u32 = unsafe {
                core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
            };
            let _ = unsafe { configure(initialized) };
        }
    } else {
        unsafe {
            core::ptr::write_unaligned(this.add(SPEED_FIELD).cast::<u32>(), 0);
            core::ptr::write_unaligned(this.add(LIMIT_FIELD).cast::<u32>(), 0);
        }
    }

    let begin: unsafe extern "thiscall" fn(*mut u8, u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
    };
    let first_result = unsafe { begin(this, RESOURCE_KEY) };
    if first_result == 0 {
        return first_result;
    }

    let float_bits = unsafe {
        core::ptr::read_unaligned(this.add(SPEED_FIELD).cast::<u32>())
    };
    let finish: unsafe extern "thiscall" fn(*mut u8, u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
    };
    let object = unsafe { finish(this, RESOURCE_KEY) };
    let apply: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(3) as usize)
    };
    unsafe { apply(object, float_bits) }
}
