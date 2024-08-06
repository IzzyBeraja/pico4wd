#![no_std]
#![no_main]

use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm_back_right_wheel = &mut pwm_slices.pwm5;
    pwm_back_right_wheel.set_ph_correct();
    pwm_back_right_wheel.enable();
    let pwm_front_right_wheel = &mut pwm_slices.pwm6;
    pwm_front_right_wheel.set_ph_correct();
    pwm_front_right_wheel.enable();
    let pwm_back_left_wheel = &mut pwm_slices.pwm7;
    pwm_back_left_wheel.set_ph_correct();
    pwm_back_left_wheel.enable();
    let pwm_front_left_wheel = &mut pwm_slices.pwm0;
    pwm_front_left_wheel.set_ph_correct();
    pwm_front_left_wheel.enable();

    let back_right_a = &mut pwm_back_right_wheel.channel_a;
    back_right_a.output_to(pins.gpio10);
    let back_right_b = &mut pwm_back_right_wheel.channel_b;
    back_right_b.output_to(pins.gpio11);
    let front_left_a = &mut pwm_front_right_wheel.channel_a;
    front_left_a.output_to(pins.gpio12);
    let front_left_b = &mut pwm_front_right_wheel.channel_b;
    front_left_b.output_to(pins.gpio13);
    let back_left_a = &mut pwm_back_left_wheel.channel_a;
    back_left_a.output_to(pins.gpio14);
    let back_left_b = &mut pwm_back_left_wheel.channel_b;
    back_left_b.output_to(pins.gpio15);
    let front_right_a = &mut pwm_front_left_wheel.channel_a;
    front_right_a.output_to(pins.gpio16);
    let front_right_b = &mut pwm_front_left_wheel.channel_b;
    front_right_b.output_to(pins.gpio17);

    loop {
        front_right_a.set_duty_cycle(65535).unwrap();
        front_right_b.set_duty_cycle(0).unwrap();
        back_right_a.set_duty_cycle(65535).unwrap();
        back_right_b.set_duty_cycle(0).unwrap();
        front_left_a.set_duty_cycle(0).unwrap();
        front_left_b.set_duty_cycle(65535).unwrap();
        back_left_a.set_duty_cycle(0).unwrap();
        back_left_b.set_duty_cycle(65535).unwrap();
    }
}
