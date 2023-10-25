#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(unused_imports, clippy::single_component_path_imports)]

use {defmt::*, defmt_rtt, panic_probe};

use embassy_executor::{Spawner, _export::StaticCell};
use embassy_time::{Duration, Timer};

#[cfg(all(target_os = "none", target_arch = "xtensa", target_vendor = "unknown"))]
use esp32_hal::{embassy::executor::Executor, entry};

#[cfg(target_arch = "xtensa")]
#[entry]
async fn main() -> ! {
    Executor::new().run(demo_main)
}

#[cfg(not(target_arch = "xtensa"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    demo_main(spawner).await.unwrap();
    panic!()
}

async fn demo_main(spawner: Spawner) -> anyhow::Result<()> {
    panic!()
}

#[embassy_macros::task]
async fn blink() {
    panic!()
}
