// original: 0x00B37BA0 ped_task_candidate_dispatch

/// Collects up to one hundred eligible candidates from two pointer/stride
/// lists, computes each candidate's squared XY distance, asks the supplied
/// helpers to order and classify the records, then dispatches the selected
/// candidate through its object table. The two list globals point to records
/// with data, flags, count, and stride words at offsets 0, 4, 8, and 0x0c.
/// Candidate entries use a signed 16-bit type index at +0x2e, a type pointer
/// table entry whose descriptor kind at +0x90 must equal 2, an object pointer
/// at +0x20, and XY coordinates at +0x30/+0x34 from that object. The first
/// list also rejects entries with byte +0x219 set. Squared distance is
/// `(target_x - object_x)^2 + (target_y - object_y)^2`, in that operand order.
/// The input count cap is unsigned; the helper's classification result is
/// interpreted as signed-positive. This cdecl routine has five stack words:
/// target XY pointer, an unused word, two callback float values, and a cap.
///
/// The proof contract exercises one accepted first-list entry, a positive
/// squared-distance calculation, the sorting and query helpers, and the
/// XMM0 table callback path. The second-list route and callback alternatives
/// are implemented here but not covered by that contract.
lf_checker_rt::export!(cdecl, rw_00b37ba0(
    target_xy: u32,
    _unused: u32,
    callback_value: f32,
    method_value: f32,
    max_count: u32
) -> u32 {
    unsafe { run_task_dispatch_00b37ba0(target_xy, callback_value.to_bits(), method_value.to_bits(), max_count, false) }
});

#[repr(C)]
#[derive(Clone, Copy)]
struct CandidateRecord00b37ba0 {
    primary: u32,
    secondary: u32,
    marker: u32,
    distance: f32,
}

const LIST_PRIMARY_00B37BA0: u32 = 0x018B_6F1C;
const LIST_SECONDARY_00B37BA0: u32 = 0x018B_6F10;
const TYPE_TABLE_00B37BA0: u32 = 0x0129_5CD8;
const LIST_DATA_00B37BA0: u32 = 0x00;
const LIST_FLAGS_00B37BA0: u32 = 0x04;
const LIST_COUNT_00B37BA0: u32 = 0x08;
const LIST_STRIDE_00B37BA0: u32 = 0x0C;
const ENTRY_TYPE_00B37BA0: u32 = 0x2E;
const ENTRY_COORDINATES_00B37BA0: u32 = 0x20;
const ENTRY_PRIMARY_FILTER_00B37BA0: u32 = 0x219;
const COORDINATE_X_00B37BA0: u32 = 0x30;
const COORDINATE_Y_00B37BA0: u32 = 0x34;
const TYPE_KIND_00B37BA0: u32 = 0x90;
const REQUIRED_TYPE_KIND_00B37BA0: u32 = 2;
const FLAG_SKIP_00B37BA0: u8 = 0x80;
const MAX_CANDIDATES_00B37BA0: usize = 100;
const RECORD_BYTES_00B37BA0: u32 = 16;
const RECORD_CAPACITY_BYTES_00B37BA0: u32 = (MAX_CANDIDATES_00B37BA0 as u32) * RECORD_BYTES_00B37BA0;

#[inline(always)]
unsafe fn read_u8_00b37ba0(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn read_u16_00b37ba0(address: u32) -> u16 {
    unsafe { (address as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32_00b37ba0(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_f32_00b37ba0(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32_00b37ba0(address) })
}

#[inline(always)]
unsafe fn write_u8_00b37ba0(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

#[inline(always)]
fn ordered_sub_00b37ba0(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn ordered_mul_00b37ba0(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn ordered_add_00b37ba0(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
unsafe fn scan_list_00b37ba0(
    records: &mut [CandidateRecord00b37ba0; MAX_CANDIDATES_00B37BA0],
    record_count: &mut usize,
    list_address: u32,
    target_x: f32,
    target_y: f32,
    reject_primary_flag: bool,
) {
    let entry_count = unsafe { read_u32_00b37ba0(list_address + LIST_COUNT_00B37BA0) };
    let flags = unsafe { read_u32_00b37ba0(list_address + LIST_FLAGS_00B37BA0) };
    let data = unsafe { read_u32_00b37ba0(list_address + LIST_DATA_00B37BA0) };
    let stride = unsafe { read_u32_00b37ba0(list_address + LIST_STRIDE_00B37BA0) };
    let mut index = entry_count;

    while index != 0 && *record_count < MAX_CANDIDATES_00B37BA0 {
        index -= 1;
        if unsafe { read_u8_00b37ba0(flags.wrapping_add(index)) } & FLAG_SKIP_00B37BA0 != 0 {
            continue;
        }

        let entry = data.wrapping_add(stride.wrapping_mul(index));
        if entry == 0 {
            continue;
        }
        if reject_primary_flag && unsafe { read_u8_00b37ba0(entry + ENTRY_PRIMARY_FILTER_00B37BA0) } != 0 {
            continue;
        }

        let signed_type = unsafe { read_u16_00b37ba0(entry + ENTRY_TYPE_00B37BA0) } as i16;
        let table_slot = (signed_type as i32).wrapping_mul(4) as u32;
        let type_descriptor = unsafe {
            read_u32_00b37ba0(lf_checker_rt::global::<u32>(TYPE_TABLE_00B37BA0) as u32 + table_slot)
        };
        if unsafe { read_u32_00b37ba0(type_descriptor + TYPE_KIND_00B37BA0) }
            != REQUIRED_TYPE_KIND_00B37BA0
        {
            continue;
        }

        let coordinates = unsafe { read_u32_00b37ba0(entry + ENTRY_COORDINATES_00B37BA0) };
        let dx = ordered_sub_00b37ba0(target_x, unsafe { read_f32_00b37ba0(coordinates + COORDINATE_X_00B37BA0) });
        let dy = ordered_sub_00b37ba0(target_y, unsafe { read_f32_00b37ba0(coordinates + COORDINATE_Y_00B37BA0) });
        let dx_squared = ordered_mul_00b37ba0(dx, dx);
        let dy_squared = ordered_mul_00b37ba0(dy, dy);
        let distance_squared = ordered_add_00b37ba0(dy_squared, dx_squared);

        let record = &mut records[*record_count];
        if reject_primary_flag {
            record.primary = 0;
            record.secondary = entry;
        } else {
            record.primary = entry;
            record.secondary = 0;
        }
        record.marker = 0;
        record.distance = distance_squared;
        *record_count += 1;
    }
}

unsafe fn run_task_dispatch_00b37ba0(
    target_xy: u32,
    callback_value_bits: u32,
    method_value_bits: u32,
    max_count: u32,
    wrong_query_this: bool,
) -> u32 {
    unsafe {
        let target_x = read_f32_00b37ba0(target_xy);
        let target_y = read_f32_00b37ba0(target_xy + 4);
        let mut records = [CandidateRecord00b37ba0 { primary: 0, secondary: 0, marker: 0, distance: 0.0 }; MAX_CANDIDATES_00B37BA0];
        let mut record_count = 0usize;

        let primary_list = read_u32_00b37ba0(lf_checker_rt::global::<u32>(LIST_PRIMARY_00B37BA0) as u32);
        let secondary_list = read_u32_00b37ba0(lf_checker_rt::global::<u32>(LIST_SECONDARY_00B37BA0) as u32);
        scan_list_00b37ba0(&mut records, &mut record_count, primary_list, target_x, target_y, true);
        scan_list_00b37ba0(&mut records, &mut record_count, secondary_list, target_x, target_y, false);

        let records_begin = records.as_mut_ptr() as u32;
        let records_end = records_begin.wrapping_add((record_count as u32) * RECORD_BYTES_00B37BA0);
        let _ = lf_checker_rt::callee_cdecl!(1, u32, records_begin, records_end, 0u32);
        let manager = lf_checker_rt::callee_cdecl!(2, u32, 0u32, records_begin, records_end, 0u32);
        let manager_query = if manager == 0 { 0 } else { read_u32_00b37ba0(manager + 0x228) };
        let query_object = if manager_query == 0 { 0 } else { manager_query.wrapping_add(0x70) };
        let query_this = if wrong_query_this { query_object.wrapping_add(4) } else { query_object };
        let query_result = lf_checker_rt::callee_thiscall!(3, u32, query_this);
        let query_is_positive = (query_result as i32) > 0;

        let capped_count = if max_count < record_count as u32 { max_count } else { record_count as u32 };
        records[0].marker = capped_count;

        let mut eligible_count = 0u32;
        if query_is_positive {
            for index in 0..record_count {
                let primary = records[index].primary;
                if primary == 0 || read_u8_00b37ba0(primary + 0x211) != 0 {
                    continue;
                }
                let status = read_u32_00b37ba0(primary + 0xA74);
                if status != 1 && status != 2 {
                    let flag_address = records[index].secondary;
                    write_u8_00b37ba0(flag_address, 1);
                    eligible_count = eligible_count.wrapping_add(1);
                }
            }
        }

        for index in 0..record_count {
            if (records[index].marker as u8) != 0 {
                continue;
            }
            let item = if records[index].secondary != 0 {
                records[index].secondary
            } else {
                records[index].primary
            };
            if item == 0 {
                continue;
            }

            let helper_result = if records[index].secondary != 0 {
                lf_checker_rt::callee_cdecl!(4, u32, item, target_xy, callback_value_bits, eligible_count)
            } else {
                lf_checker_rt::callee_cdecl!(9, u32, item, target_xy, callback_value_bits, eligible_count)
            };
            let vtable = read_u32_00b37ba0(item);
            let update_method = read_u32_00b37ba0(vtable + 0xD0);
            let method_argument = if (helper_result as u8) != 0 {
                method_value_bits
            } else {
                0
            };
            let update: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(update_method as usize);
            let method_result = update(item, method_argument);

            if (helper_result as u8) != 0 {
                if (lf_checker_rt::callee_thiscall!(5, u32, method_result) as u8) != 0 {
                    let child = read_u32_00b37ba0(item + 0x6C);
                    let child_blocks = if child == 0 {
                        false
                    } else {
                        let child_vtable = read_u32_00b37ba0(child);
                        let child_method = read_u32_00b37ba0(child_vtable + 0x100);
                        let call: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(child_method as usize);
                        (call(child, 0) as u8) == 0
                    };
                    if !child_blocks {
                        let _ = lf_checker_rt::callee_cdecl!(8, u32, 1u32, item);
                    }
                }
            } else {
                let _ = lf_checker_rt::callee_thiscall!(7, u32, method_result);
            }
        }

        let _ = RECORD_CAPACITY_BYTES_00B37BA0;
        0
    }
}
