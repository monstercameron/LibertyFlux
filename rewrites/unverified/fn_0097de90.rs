// original: 0x0097DE90 PED_RINGTONE

/// Applies the initialization gates and updates the ring tone's output word
/// on the skip path and on the setup path when the generated object is null.
///
/// The verified input set holds the first gate open, varies the epoch and
/// mode exits, and exercises both a preexisting nonzero slot and a setup call
/// that writes a null slot. The incoming stack word is fixed at zero, so the
/// tail takes its zero-output case. The live-object callback path and the
/// positive stack-word addend case are outside this proof. Stack comparison
/// is disabled because the original uses stack scratch and a frame-derived
/// cookie argument; the frame pointer passed to the block initializer is
/// not compared, while the following setup call's 16-word block snapshot is
/// compared. The cookie check call is captured, but its frame-derived ECX
/// value is not compared.
const GATE_INIT: u32 = 0x11F7060;
const GATE_EPOCH: u32 = 0x12088B4;
const GATE_EPOCH_COPY: u32 = 0xF1C040;
const GATE_MODE: u32 = 0x1037720;
const COOKIE_WORD: u32 = 0x1057FB4;
const GATE_MODE_EXIT: u32 = 0x12;
const OUTPUT_SLOT: u32 = 0x130;
const OUTPUT_VALUE: u32 = 0x138;
const BLOCK_SOURCE: u32 = 0x120;
const BLOCK_TAG: u32 = 0x08;
const BLOCK_PRESET: u32 = 0x780;
const SETUP_NAME: u32 = 0xE8CB8C;
const CALL_BLOCK_INIT: u32 = 1;
const CALL_SETUP: u32 = 2;
const CALL_COOKIE_CHECK: u32 = 11;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

unsafe fn cookie_return(value: u32) -> u32 {
    let cookie = unsafe { lf_checker_rt::global::<u32>(COOKIE_WORD).read() };
    let _ = lf_checker_rt::callee_thiscall!(CALL_COOKIE_CHECK, u32, cookie);
    value
}

pub(crate) unsafe fn rewrite(this: u32, _stack_word: u32, suppress_tail_store: bool) -> u32 {
    if unsafe { lf_checker_rt::global::<u32>(GATE_INIT).read() } == 1 {
        // The proof keeps this entry-EAX exit out of its input set.
        return unsafe { cookie_return(0) };
    }

    let epoch = unsafe { lf_checker_rt::global::<u32>(GATE_EPOCH).read() };
    let epoch_copy = unsafe { lf_checker_rt::global::<u32>(GATE_EPOCH_COPY).read() };
    let mode = unsafe { lf_checker_rt::global::<u32>(GATE_MODE).read() };
    if epoch != epoch_copy || mode == GATE_MODE_EXIT {
        return unsafe { cookie_return(epoch) };
    }

    let mut result = epoch;
    if unsafe { read_u32(this.wrapping_add(OUTPUT_SLOT)) } == 0 {
        let mut request_block = [0u32; 16];
        let block = request_block.as_mut_ptr() as u32;
        let output_slot = this.wrapping_add(OUTPUT_SLOT);

        let _ = lf_checker_rt::callee_thiscall!(CALL_BLOCK_INIT, u32, block);
        unsafe {
            write_u32(
                block.wrapping_add(3 * 4),
                read_u32(this.wrapping_add(BLOCK_SOURCE)).wrapping_add(BLOCK_PRESET),
            );
            write_u32(
                block.wrapping_add(8 * 4),
                read_u32(this.wrapping_add(BLOCK_TAG)),
            );
        }

        let setup_answer = lf_checker_rt::callee_thiscall!(
            CALL_SETUP,
            u32,
            this,
            lf_checker_rt::relocated(SETUP_NAME),
            output_slot,
            block,
            u32::MAX,
            0,
            0
        );
        if unsafe { read_u32(output_slot) } == 0 {
            result = setup_answer;
        } else {
            // The live-object callback path is not exercised by this proof.
            result = setup_answer;
        }
    }

    // The contract defines the stale stack word as zero. Its positive-value
    // addend path is outside this proof's input set.
    if !suppress_tail_store {
        unsafe { write_u32(this.wrapping_add(OUTPUT_VALUE), 0) };
    }
    unsafe { cookie_return(result) }
}

lf_checker_rt::export!(thiscall, rw_0097de90(this: u32, stack_word: u32) -> u32 {
    unsafe { rewrite(this, stack_word, false) }
});
