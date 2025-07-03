#![no_std]
#![no_main]

use embassy_rp::i2c::{I2c, InterruptHandler};
use embassy_rp::peripherals::I2C0;
use embassy_time::{Delay, Duration, Timer};
use lcd_lcm1602_i2c::async_lcd::Lcd;
use {defmt_rtt as _, panic_probe as _};

embassy_rp::bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

async fn init_lcd<'a>(
    bus: &'a mut I2c<'a, I2C0, embassy_rp::i2c::Async>,
    delay: &'a mut Delay,
) -> Result<Lcd<'a, I2c<'a, I2C0, embassy_rp::i2c::Async>, Delay>, embassy_rp::i2c::Error> {
    const LCD_ADDRESS: u8 = 0x27;

    let lcd_scr = Lcd::new(bus, delay)
        .with_address(LCD_ADDRESS)
        .with_cursor_on(false)
        .with_rows(2);

    let lcd_scr = lcd_scr.init().await.unwrap(); // `init` returns ownership
    Ok(lcd_scr)
}

async fn lcd_display<'a>(
    lcd: &mut Lcd<'a, I2c<'a, I2C0, embassy_rp::i2c::Async>, Delay>,
    msg: &str,
) {
    lcd.clear().await.unwrap();
    lcd.write_str("Hello Rust!").await.unwrap();

    lcd.set_cursor(1, 0).await.unwrap(); // Move to the second line
    lcd.write_str(msg).await.unwrap();

    Timer::after(Duration::from_secs(2)).await;
}

#[embassy_executor::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut bus = I2c::new_async(
        p.I2C0,
        p.PIN_1,
        p.PIN_0,
        Irqs,
        embassy_rp::i2c::Config::default(),
    );
    let mut delay = Delay;

    // Initialize the LCD
    let mut lcd = init_lcd(&mut bus, &mut delay).await.unwrap();

    // Display messages
    loop {
        lcd_display(&mut lcd, "Hello World!").await;
    }
}
