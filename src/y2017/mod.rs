mod advent01;
mod advent02;

use std::collections::HashMap;
use crate::utils::{Solve, add_default_to_collection};


pub(crate) fn collect_solutions() -> HashMap<u8, Box<dyn Solve>>{
    let mut advents: HashMap<u8, Box<dyn Solve>> = HashMap::new();
    add_default_to_collection::<advent01::Advent>(&mut advents);
    add_default_to_collection::<advent02::Advent>(&mut advents);
    // add_default_to_collection::<advent03::Advent>(&mut advents);
    // add_default_to_collection::<advent04::Advent>(&mut advents);
    // add_default_to_collection::<advent05::Advent>(&mut advents);
    // add_default_to_collection::<advent06::Advent>(&mut advents);
    // add_default_to_collection::<advent07::Advent>(&mut advents);
    // add_default_to_collection::<advent08::Advent>(&mut advents);
    // add_default_to_collection::<advent09::Advent>(&mut advents);
    // add_default_to_collection::<advent10::Advent>(&mut advents);
    // add_default_to_collection::<advent11::Advent>(&mut advents);
    // add_default_to_collection::<advent12::Advent>(&mut advents);
    // add_default_to_collection::<advent13::Advent>(&mut advents);
    // add_default_to_collection::<advent14::Advent>(&mut advents);
    // add_default_to_collection::<advent15::Advent>(&mut advents);
    // add_default_to_collection::<advent16::Advent>(&mut advents);
    // add_default_to_collection::<advent17::Advent>(&mut advents);
    // add_default_to_collection::<advent18::Advent>(&mut advents);
    // add_default_to_collection::<advent19::Advent>(&mut advents);
    // add_default_to_collection::<advent20::Advent>(&mut advents);
    // add_default_to_collection::<advent21::Advent>(&mut advents);
    // add_default_to_collection::<advent22::Advent>(&mut advents);
    // add_default_to_collection::<advent23::Advent>(&mut advents);
    // add_default_to_collection::<advent24::Advent>(&mut advents);
    // add_default_to_collection::<advent25::Advent>(&mut advents);
    advents
}
