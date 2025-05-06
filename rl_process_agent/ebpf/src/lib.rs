#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, kprobe},
    maps::PerfEventArray,
    programs::KProbeContext,
    helpers::bpf_get_current_comm,
};

#[repr(C)]
pub struct ActivityEvent {
    pub pid: u32,
    pub comm: [u8; 16],
    pub activity: u8, // 0 = exec, 1 = read, 2 = connect, 3 = cpu
}

#[map]
static mut EVENTS: PerfEventArray<ActivityEvent> = PerfEventArray::with_max_entries(1024);

#[kprobe(name = "trace_exec")]
pub fn trace_exec(ctx: KProbeContext) -> u32 {
    send_event(ctx, 0)
}

#[kprobe(name = "trace_read")]
pub fn trace_read(ctx: KProbeContext) -> u32 {
    send_event(ctx, 1)
}

#[kprobe(name = "trace_connect")]
pub fn trace_connect(ctx: KProbeContext) -> u32 {
    send_event(ctx, 2)
}

#[kprobe(name = "trace_switch")]
pub fn trace_switch(ctx: KProbeContext) -> u32 {
    send_event(ctx, 3)
}

fn send_event(ctx: KProbeContext, activity: u8) -> u32 {
    let mut comm = [0u8; 16];
    unsafe {
        bpf_get_current_comm(&mut comm);
    }

    let event = ActivityEvent {
        pid: ctx.pid() as u32,
        comm,
        activity,
    };

    unsafe {
        EVENTS.output(&ctx, &event, core::mem::size_of::<ActivityEvent>() as u32);
    }

    0
}