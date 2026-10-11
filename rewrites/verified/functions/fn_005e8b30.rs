// original: 0x005E8B30 CTaskComplexWanderCop::~CTaskComplexWanderCop__deleting

/// Destroy a wander-cop task and optionally release its timing-pool slot.
///
/// `this` is the task object and `flags` is the deleting-destructor argument;
/// bit 0 requests slot release. The object stores its current subtask pointer
/// at `+0x80`. This replaces the task vtable, calls the subtask's virtual
/// destructor with argument 1 when that pointer is non-null, then calls the
/// base wander-task destructor. When release is requested, the global timing
/// pool supplies a base object pointer, a byte table, a signed stride, a
/// signed low-water mark and a count. The object's signed slot index is
/// `(this - pool_base) / stride` with truncation toward zero; the table byte
/// gets bit 7 set, the low-water mark becomes the lesser signed index, and
/// the count wraps down by one. The function returns `this` in EAX and pops
/// one stack word. The contract covers valid nonzero pool strides and indices
/// that stay inside the supplied table.
lf_checker_rt::export!(thiscall, rw_005e8b30(this: u32, flags: u32) -> u32 {
    const SUBTASK_PTR_OFFSET: u32 = 0x80;
    const WANDER_COP_VTABLE_VA: u32 = 0x00FE1314;
    const TIMING_POOL_GLOBAL_VA: u32 = 0x0167E2A0;
    const POOL_BASE_OFFSET: u32 = 0x00;
    const POOL_TABLE_OFFSET: u32 = 0x04;
    const POOL_STRIDE_OFFSET: u32 = 0x0c;
    const POOL_LOW_WATER_OFFSET: u32 = 0x10;
    const POOL_COUNT_OFFSET: u32 = 0x14;
    const RELEASE_SLOT_BIT: u32 = 1;

    let subtask = unsafe {
        (this.wrapping_add(SUBTASK_PTR_OFFSET) as *const u32).read_unaligned()
    };
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(WANDER_COP_VTABLE_VA));
    }
    if subtask != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, subtask, 1);
    }

    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & RELEASE_SLOT_BIT != 0 {
        let pool = unsafe {
            lf_checker_rt::global::<u32>(TIMING_POOL_GLOBAL_VA).read_unaligned()
        };
        let pool_base = unsafe {
            (pool.wrapping_add(POOL_BASE_OFFSET) as *const u32).read_unaligned()
        };
        let table = unsafe {
            (pool.wrapping_add(POOL_TABLE_OFFSET) as *const u32).read_unaligned()
        };
        let stride = unsafe {
            (pool.wrapping_add(POOL_STRIDE_OFFSET) as *const u32).read_unaligned() as i32
        };
        let slot = this.wrapping_sub(pool_base) as i32 / stride;
        let table_byte = table.wrapping_add(slot as u32) as *mut u8;
        unsafe {
            table_byte.write(table_byte.read() | 0x80);
        }

        let low_water = unsafe {
            (pool.wrapping_add(POOL_LOW_WATER_OFFSET) as *const u32).read_unaligned() as i32
        };
        if slot < low_water {
            unsafe {
                (pool.wrapping_add(POOL_LOW_WATER_OFFSET) as *mut u32)
                    .write_unaligned(slot as u32);
            }
        }
        let count = unsafe {
            (pool.wrapping_add(POOL_COUNT_OFFSET) as *const u32).read_unaligned()
        };
        unsafe {
            (pool.wrapping_add(POOL_COUNT_OFFSET) as *mut u32)
                .write_unaligned(count.wrapping_sub(1));
        }
    }
    this
});
