use embassy_rp::i2c::I2c;
use embassy_time::Delay;
use lcd_lcm1602_i2c::async_lcd::Lcd;

pub async fn init_lcd<'a>(
    bus: &'a mut I2c<'a, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Async>,
    delay: &'a mut Delay,
) -> Result<Lcd<'a, I2c<'a, _, _>, Delay>, embassy_rp::i2c::Error> {
    const LCD_ADDR: u8 = 0x27;
    let lcd = Lcd::new(bus, delay)
        .with_address(LCD_ADDR)
        .with_cursor_on(false)
        .with_rows(2);
    Ok(lcd.init().await.unwrap())
}

pub async fn welcome_message(
    lcd: &mut Lcd<'_, I2c<'_, _, _>, Delay>,
) {
    lcd.clear().await.unwrap();
    lcd.write_str("Rust Weather!").await.unwrap();
}
