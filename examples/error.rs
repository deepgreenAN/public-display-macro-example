#![allow(dead_code)]

use public_display::PublicDisplay;

struct NotDebug;

#[derive(PublicDisplay)]
struct HasError {
    // pub not_debug: NotDebug, // ここがエラーになる．
}

fn main() {}
