use mork_common::constants::MAX_THREAD_PIRO;
use mork_hal::timer::set_next_trigger;
use mork_kernel_state::KernelSafeAccessData;
use mork_task::task::TaskContext;
use mork_task::task_state::ThreadStateEnum;
use mork_task::TIME_SLICE;

pub fn handle_timer_interrupt(kernel_state: &mut KernelSafeAccessData) {
    let mut current = kernel_state.current_task.take().unwrap();
    current.state = ThreadStateEnum::ThreadStateRestart;
    let need_resume = timer_kick(&mut current);
    let is_idle = current.prio == MAX_THREAD_PIRO;
    if is_idle {
        kernel_state.idle_task = Some(current);
    } else {
        if need_resume {
            kernel_state.scheduler.enqueue_front(current);
        } else {
            kernel_state.scheduler.enqueue_back(current);
        }
    }

    set_next_trigger();
}

fn timer_kick(current: &mut TaskContext) -> bool {
    if current.time_slice == 1 {
        current.time_slice = TIME_SLICE;
        return false;
    }
    current.time_slice -= 1;
    true
}