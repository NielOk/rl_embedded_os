#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, tracepoint},
    maps::RingBuf,
    programs::TracePointContext,
    helpers::{bpf_get_current_pid_tgid, bpf_probe_read_user_str},
};

#[repr(C)]
pub struct FileOpenEvent {
    pub pid: u32,
    pub filename: [u8; 256],
}

#[map(name = "EVENTS")]
static mut EVENTS: RingBuf<FileOpenEvent> = RingBuf::with_max_entries(1024);

#[tracepoint(name = "trace_openat")]
pub fn trace_openat(ctx: TracePointContext) -> u32 {
    match try_trace_openat(&ctx) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

fn try_trace_openat(ctx: &TracePointContext) -> Result<(), ()> {
    let pid_tgid = unsafe { bpf_get_current_pid_tgid() };
    let pid = (pid_tgid & 0xFFFF_FFFF) as u32;

    // For sys_enter_openat, the second argument is `const char __user *filename`
    let filename_ptr: *const u8 = ctx.read_at::<*const u8>(16).map_err(|_| ())?;

    let mut event = FileOpenEvent {
        pid,
        filename: [0u8; 256],
    };

    unsafe {
        bpf_probe_read_user_str(&mut event.filename, filename_ptr).map_err(|_| ())?;
        EVENTS.output(ctx, &event, core::mem::size_of::<FileOpenEvent>() as u32)
            .map_err(|_| ())?;
    }

    Ok(())
}