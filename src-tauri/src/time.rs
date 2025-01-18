use std::time::Instant;

pub struct time {
    start_time: Instant,
}

pub fn elapsed_time(user_time: &time) -> i32 {
    user_time.start_time.elapsed().as_secs() as i32 
}

pub fn start_time() -> time {
    let user_time = time {
        start_time: Instant::now()
    };
    user_time
}
