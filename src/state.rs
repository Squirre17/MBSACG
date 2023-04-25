
use std::collections::VecDeque;
pub struct States {
    que : VecDeque<State>,
}
/*
    afl->virgin_bits = ck_realloc(afl->virgin_bits, map_size);
    afl->virgin_tmout = ck_realloc(afl->virgin_tmout, map_size);
    afl->virgin_crash = ck_realloc(afl->virgin_crash, map_size);
    afl->var_bytes = ck_realloc(afl->var_bytes, map_size);
    afl->top_rated = ck_realloc(afl->top_rated, map_size * sizeof(void *));
    afl->clean_trace = ck_realloc(afl->clean_trace, map_size);
    afl->clean_trace_custom = ck_realloc(afl->clean_trace_custom, map_size);
    afl->first_trace = ck_realloc(afl->first_trace, map_size);
    afl->map_tmp_buf = ck_realloc(afl->map_tmp_buf, map_size);
 */

pub struct State {
    
}
impl State {
    pub fn new() -> Self {
        State {  }
    }
}