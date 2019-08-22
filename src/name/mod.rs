mod first_names;
mod last_names;
mod middle_names;

use first_names::FIRST_NAMES;
use last_names::LAST_NAMES;
use middle_names::MIDDLE_NAMES;
use super::util;

pub fn full() -> String {
    if rand::random() {
        return format!("{} {} {}", first(), middle(), last());
    } else {
        return format!("{} {}", first(), last());
    }
}
pub fn first() -> String {
    let index: usize = util::rand_index(FIRST_NAMES.len());
    return FIRST_NAMES[index].to_owned();
}

pub fn last() -> String{
    let index: usize = util::rand_index(LAST_NAMES.len());
    return LAST_NAMES[index].to_owned();
}

fn middle() -> String {
    let index: usize = util::rand_index(MIDDLE_NAMES.len());
    return MIDDLE_NAMES[index].to_owned();
}