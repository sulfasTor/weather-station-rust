//! This example shows how to communicate asynchronous using i2c with external chip.
//!
//! It's using embassy's functions directly instead of traits from embedded_hal_async::i2c::I2c.
//! While most of i2c devices are addressed using 7 bits, an extension allows 10 bits too.

#![no_std]
#![no_main]
use embassy_rp::i2c::InterruptHandler;
use embassy_time::{Delay, Timer, Duration};
use lcd_lcm1602_i2c::async_lcd::Lcd;
use {defmt_rtt as _, panic_probe as _};

embassy_rp::bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<embassy_rp::peripherals::I2C0>;
});

#[embassy_executor::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    const LCD_ADDRESS: u8 = 0x26;
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_0;
    let scl = p.PIN_1;
    let config = embassy_rp::i2c::Config::default();
    let mut bus = embassy_rp::i2c::I2c::new_async(p.I2C0, scl, sda, Irqs, config);
    let mut delay = Delay;

    let mut lcd_scr = Lcd::new(&mut bus, &mut delay)
        .with_address(LCD_ADDRESS)
        .with_cursor_on(false)
        .with_rows(2);

    // Initialize the LCD
    lcd_scr = lcd_scr.init().await.unwrap();
    lcd_scr.clear().await.unwrap();

    // Write a message to the LCD
    lcd_scr.write_str("Hello Rust!").await.unwrap();

    // Main loop: write messages with delays
    loop {
        lcd_scr.set_cursor(0, 0).await.unwrap(); // Move to the second line
        lcd_scr.write_str("Async + LCD").await.unwrap();

        Timer::after(Duration::from_secs(2)).await;

        lcd_scr.clear().await.unwrap();
        lcd_scr.write_str("Embassy Rocks!").await.unwrap();

        Timer::after(Duration::from_secs(2)).await;
    }
}
