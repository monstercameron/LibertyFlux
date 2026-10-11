// original: 0x00CEBE10 CBike::vf106
//! Updates the object's random byte and timer only when its type and state
//! fields allow the update. A nonzero type other than `0xFFFF` additionally
//! requires `(state & 0x7C00) == 0x0C00`; the timer at `0x12EC` must be zero.
//!
//! If the byte at `0x12F1` is already nonzero and the low byte of the stack
//! argument is zero, the method decrements that byte and returns early.
//! Otherwise it uses the random helper's low byte masked by `0x7F`, subtracts
//! `0x6A` with byte wraparound, and stores the result. It writes timer value
//! `0x22` when the new byte's low three bits are below four, then calls the
//! updater with six stack words. Both helpers are scripted by the contract.
//! EAX and stack scratch memory are not compared: the random-helper result is
//! observed in the object and call arguments, while the temporary stack word
//! is observed in the call log. The thiscall method has one stack argument.
const TYPE_FIELD: usize = 0x0F48;
const STATE_BITS: usize = 0x28;
const TIMER_FIELD: usize = 0x12EC;
const RANDOM_FIELD: usize = 0x12F1;
const DATA_POINTER: usize = 0x20;
const RANDOM_MASK: u8 = 0x7F;
const RANDOM_BIAS: u8 = 0x6A;
const TIMER_RELOAD: u32 = 0x22;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00cebe10(this: *mut u8, request_word: u32) -> u32 {
    let type_code = unsafe {
        core::ptr::read_unaligned(this.add(TYPE_FIELD).cast::<u16>())
    };
    if type_code != 0 && type_code != u16::MAX {
        let state = unsafe { core::ptr::read_unaligned(this.add(STATE_BITS).cast::<u32>()) };
        if state & 0x7C00 != 0x0C00 {
            return 0;
        }
    }
    let timer = unsafe {
        core::ptr::read_unaligned(this.add(TIMER_FIELD).cast::<u32>())
    };
    if timer != 0 {
        return 0;
    }

    let current = unsafe { this.add(RANDOM_FIELD).read() };
    if current != 0 && request_word as u8 == 0 {
        unsafe { this.add(RANDOM_FIELD).write(current.wrapping_sub(1)) };
        return 0;
    }

    let random: unsafe extern "C" fn() -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
    };
    let random_byte = unsafe { random() } as u8;
    let next_value = (random_byte & RANDOM_MASK).wrapping_sub(RANDOM_BIAS);
    unsafe { this.add(RANDOM_FIELD).write(next_value) };
    if next_value & 7 < 4 {
        unsafe {
            core::ptr::write_unaligned(this.add(TIMER_FIELD).cast::<u32>(), TIMER_RELOAD);
        }
    }

    let data = unsafe {
        core::ptr::read_unaligned(this.add(DATA_POINTER).cast::<*mut u8>())
    };
    let update: unsafe extern "C" fn(u32, *mut u8, *mut u8, u32, u32, u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
    };
    unsafe { update(0x0C, data.wrapping_add(0x30), this, 0, 0, 0) }
}
