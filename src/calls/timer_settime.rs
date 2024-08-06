/// Arm/disarm state of per-process timer.
///
/// # Examples
///
/// ```
/// const TIMER_SIG: i32 = nc::SIGRTMAX;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, TIMER_SIG);
/// }
///
/// fn main() {
///     #[cfg(nc_has_sa_restorer)]
///     let sa = nc::sigaction_t {
///         sa_handler: handle_alarm as nc::sighandler_t,
///         sa_flags: nc::SA_RESTORER | nc::SA_RESTART,
///         sa_restorer: nc::restore::get_sa_restorer(),
///         ..nc::sigaction_t::default()
///     };
///     #[cfg(not(nc_has_sa_restorer))]
///     let sa = nc::sigaction_t {
///         sa_handler: handle_alarm as nc::sighandler_t,
///         sa_flags: nc::SA_RESTART,
///         ..nc::sigaction_t::default()
///     };
///     let ret = unsafe { nc::rt_sigaction(TIMER_SIG, Some(&sa), None) };
///     assert!(ret.is_ok());
///
///     let tid = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let mut ev = nc::sigevent_t {
///         sigev_value: nc::sigval_t {
///             sival_ptr: &tid as *const nc::itimerspec_t as usize,
///         },
///         sigev_signo: TIMER_SIG,
///         sigev_notify: nc::SIGEV_SIGNAL,
///         sigev_un: nc::sigev_un_t::default(),
///     };
///     let mut timer_id = nc::timer_t::default();
///     let ret = unsafe { nc::timer_create(nc::CLOCK_MONOTONIC, Some(&mut ev), &mut timer_id) };
///     assert!(ret.is_ok());
///     println!("timer id: {:?}", timer_id);
///
///     let flags = 0;
///     let time = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let ret = unsafe { nc::timer_settime(timer_id, flags, &time, None) };
///     assert!(ret.is_ok());
///
///     let mut cur_time = nc::itimerspec_t::default();
///     let ret = unsafe { nc::timer_gettime(timer_id, &mut cur_time) };
///     assert!(ret.is_ok());
///     println!("cur time: {:?}", cur_time);
///
///     let mask = nc::sigset_t::default();
///     let _ret = unsafe { nc::rt_sigsuspend(&mask) };
///
///     let ret = unsafe { nc::timer_delete(timer_id) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn timer_settime(
    timer_id: timer_t,
    flags: i32,
    new_value: &itimerspec_t,
    old_value: Option<&mut itimerspec_t>,
) -> Result<(), Errno> {
    let timer_id = timer_id as usize;
    let flags = flags as usize;
    let new_value_ptr = new_value as *const itimerspec_t as usize;
    let old_value_ptr = old_value.map_or(
        core::ptr::null_mut::<itimerspec_t>() as usize,
        |old_value| old_value as *mut itimerspec_t as usize,
    );
    syscall4(
        SYS_TIMER_SETTIME,
        timer_id,
        flags,
        new_value_ptr,
        old_value_ptr,
    )
    .map(drop)
}
