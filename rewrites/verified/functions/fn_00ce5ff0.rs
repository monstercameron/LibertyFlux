// original: 0x00CE5FF0 peds_task_randomized_schedule_init
/// Initialize a task record from a 20-byte source block and two signed
/// endpoint pairs stored at offsets `0/4` and `8/12`.
///
/// The first random draw selects one of four fixed tags. The first caller
/// value is copied to `this+0x24`, the readiness byte at `this+0x29` is
/// cleared, and two further draws select truncated offsets within the
/// signed endpoint spans. The task stores each low endpoint plus its offset
/// and the shared runtime base at `this+0x14` and `this+0x18`.
///
/// The source copy is performed as two 8-byte load/store pairs and one
/// 4-byte load/store pair, preserving the original order when the source
/// partially overlaps the task record. Each float multiply follows the
/// original operand order, and float-to-integer conversion truncates toward
/// zero with the x86 invalid-result value for NaN or overflow.
///
/// This is a thiscall routine with two stack arguments. It has no meaningful
/// return value; the outgoing random calls and all object writes are checked.

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

unsafe fn initialize_task(this: u32, caller_value: u32, source: u32, mutant: bool) {
    const SOURCE_HEAD_BYTES: u32 = 8;
    const SOURCE_MIDDLE_BYTES: u32 = 8;
    const SOURCE_TAIL_OFFSET: u32 = 16;
    const TASK_CALLER_VALUE: u32 = 0x24;
    const TASK_RANDOM_TAG: u32 = 0x20;
    const TASK_RANDOM_LOW: u32 = 0x14;
    const TASK_RANDOM_HIGH: u32 = 0x18;
    const TASK_READY: u32 = 0x29;
    const TAGS: [u32; 4] = [0x1A1, 0x4B3, 0x36A1, 0x4B5];

    unsafe {
        // The two vector loads are kept as load-then-store pairs so partial
        // overlap between the source and destination has the original order.
        let source_head = (source as *const u64).read_unaligned();
        (this as *mut u64).write_unaligned(source_head);
        let source_middle = (source.wrapping_add(SOURCE_HEAD_BYTES) as *const u64).read_unaligned();
        (this.wrapping_add(SOURCE_HEAD_BYTES) as *mut u64).write_unaligned(source_middle);
        let source_tail = read_u32(source.wrapping_add(SOURCE_TAIL_OFFSET));
        write_u32(this.wrapping_add(SOURCE_MIDDLE_BYTES + SOURCE_HEAD_BYTES), source_tail);
        if mutant {
            write_u32(
                this.wrapping_add(SOURCE_TAIL_OFFSET),
                read_u32(this.wrapping_add(SOURCE_TAIL_OFFSET)) ^ 1,
            );
        }

        let random_tag = lf_checker_rt::callee_cdecl!(RNG, u32,);
        let unit = multiply((random_tag & 0xFFFF) as f32, read_f32(lf_checker_rt::relocated(RNG_SCALE_VA)));
        let scaled_index = multiply(unit, read_f32(lf_checker_rt::relocated(RNG_TABLE_SCALE_VA)));
        let index = cvttss2si(scaled_index);
        // The RNG returns 15 bits, so the negative scale and truncation
        // select indexes zero through three in the local tag array.
        let tag_index = index.wrapping_neg() as usize;
        let selected_tag = TAGS[tag_index];
        write_u32(this.wrapping_add(TASK_RANDOM_TAG), selected_tag);
        write_u32(this.wrapping_add(TASK_CALLER_VALUE), caller_value);
        (this.wrapping_add(TASK_READY) as *mut u8).write(0);

        let runtime_base = lf_checker_rt::global::<u32>(RUNNING_BASE_VA).read();
        let low = read_u32(this);
        let high = read_u32(this.wrapping_add(4));
        let span = high.wrapping_sub(low) as i32 as f32;
        let random_low = lf_checker_rt::callee_cdecl!(RNG, u32,);
        let fraction = multiply(
            (random_low & 0xFFFF) as f32,
            read_f32(lf_checker_rt::relocated(RNG_SCALE_VA)),
        );
        let offset = cvttss2si(multiply(fraction, span));
        write_u32(
            this.wrapping_add(TASK_RANDOM_LOW),
            low.wrapping_add(offset as u32).wrapping_add(runtime_base),
        );

        let low = read_u32(this.wrapping_add(8));
        let high = read_u32(this.wrapping_add(12));
        let span = high.wrapping_sub(low) as i32 as f32;
        let random_high = lf_checker_rt::callee_cdecl!(RNG, u32,);
        let fraction = multiply(
            (random_high & 0xFFFF) as f32,
            read_f32(lf_checker_rt::relocated(RNG_SCALE_VA)),
        );
        let offset = cvttss2si(multiply(fraction, span));
        write_u32(
            this.wrapping_add(TASK_RANDOM_HIGH),
            low.wrapping_add(offset as u32).wrapping_add(runtime_base),
        );
    }
}

lf_checker_rt::export!(thiscall, rw_00ce5ff0(this: u32, caller_value: u32, source: u32) -> u32 {
    unsafe { initialize_task(this, caller_value, source, false) }
    0
});
