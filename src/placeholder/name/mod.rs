mod first_name;
mod last_name;
mod middle_name;

use first_name::FIRST_NAME;
use last_name::LAST_NAME;
use middle_name::MIDDLE_NAME;
use super::util;

pub fn full() -> String {
    if rand::random() {
        return format!("{} {} {}", first(), middle(), last());
    } else {
        return format!("{} {}", first(), last());
    }
}
pub fn first() -> &'static str {
    let index: usize = util::rand_index(FIRST_NAME.len());
    return FIRST_NAME[index];
}

pub fn last() -> &'static str {
    let index: usize = util::rand_index(LAST_NAME.len());
    return LAST_NAME[index];
}

fn middle() -> &'static str {
    let index: usize = util::rand_index(MIDDLE_NAME.len());
    return MIDDLE_NAME[index];
}