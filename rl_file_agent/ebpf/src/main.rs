#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, kprobe},
    maps::RingBuf,
    programs::KProbeContext,
    BpfContext,
    helpers::{bpf_get_current_pid_tgid, bpf_probe_read_user_str},
};

#[repr(C)]
pub struct FileOpenEvent {
    pub pid: u32,
    pub filename: [u8; 256],
}

#[map]
static mut EVENTS: RingBuf<FileOpenEvent> = RingBuf::with_max_entries(1024);

#[kprobe(name = "trace_open")]
pub fn trace_open(ctx: KProbeContext) -> u32 {
    let mut event = FileOpenEvent {
        pid: unsafe { bpf_get_current_pid_tgid() as u32 },
        filename: [0; 256],
    };

    // Get 2nd syscall argument (rsi on x86_64): const char *filename
    let filename_ptr: *const u8 = unsafe { ctx.arg(1).unwrap_or(core::ptr::null()) as *const u8 };

    if !filename_ptr.is_null() {
        unsafe {
            let res = bpf_probe_read_user_str(&mut event.filename, filename_ptr);
            if res.is_ok() {
                let _ = EVENTS.output(&ctx, &event, core::mem::size_of::<FileOpenEvent>() as u32);
            }
        }
    }

    0
}