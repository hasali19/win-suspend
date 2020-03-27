use libc::c_int;

fn main() {
    let mut hibernate = false;

    if std::env::args().any(|arg| arg == "-H" || arg == "--hibernate") {
        hibernate = true;
    }

    unsafe {
        SetSuspendState(hibernate as c_int, 0, 0);
    }
}

#[link(name = "PowrProf")]
extern "C" {
    fn SetSuspendState(hibernate: c_int, force: c_int, wakeup_events_disabled: c_int) -> c_int;
}
