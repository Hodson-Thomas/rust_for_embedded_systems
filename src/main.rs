#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use cortex_m::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// CPU frequency (12.5MHz by default)
const CPU_FREQ: u32 = 12_500_000;
// System tick - control and status register
const SYST_CSR: u32 = 0xE000E010;
// System tick - reload value register
const SYST_RVR: u32 = 0xE000E014;
// System tick - current value register
const SYST_CVR: u32 = 0xE000E018;
// Run-Mode clock configuration (RCC)
const RCC: u32 = 0x400FE060;
// Prescaler value
const SYST_SYSDIV_16: u32 = 0xF;
const SYST_SYSDIV_12: u32 = 0xB;

#[entry]
fn main() -> ! {
    hprintln!("Starting program !");

    // Set the prescaler value
    unsafe {
        let sysdiv = SYST_SYSDIV_16 << 23; // <- default
        // let sysdiv = SYST_SYSDIV_12 << 23;
        let origin = *(RCC as *const u32);
        let mask = !0b1111 << 23;
        let rcc = (origin & mask) | sysdiv;
        *(RCC as *mut u32) = rcc;
    }

    let sleep_dur = CPU_FREQ;

    unsafe {
        // Set the timer duration
        *(SYST_RVR as *mut u32) = sleep_dur;
        // Clear the current value register
        *(SYST_CVR as *mut u32) = 0;
        // Enable the timer
        *(SYST_CSR as *mut u32) = 0b111;
    }

    loop {
        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    hprintln!("An exception has occured, wake up :)");
}