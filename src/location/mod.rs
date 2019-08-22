mod places;

use places::PLACES;
use super::util;

pub fn place() -> String {
    let index: usize = util::rand_index(PLACES.len());
    return PLACES[index].to_owned();
}