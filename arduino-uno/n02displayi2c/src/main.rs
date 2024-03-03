#![no_std]
#![no_main]
// With I2C
use liquidcrystal_i2c_rs::{Lcd,Display,Backlight};
use panic_halt as _;


static  LCD_ADDRESS: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {

    //use arduino_hal::{gpio::Gpio, i2c::I2c};
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut delay = arduino_hal::Delay::new();

    let sda = pins.a4.into_pull_up_input();
    let scl = pins.a5.into_pull_up_input();

    
    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        sda,
        scl,
        50000,
    );

    let mut lcd = Lcd::new(&mut i2c, LCD_ADDRESS, &mut delay).unwrap();
    
    lcd.set_display(Display::On).unwrap();
    lcd.set_backlight(Backlight::On).unwrap();
    lcd.set_cursor_position(4,0);
    lcd.print("rust!").unwrap();    

    lcd.set_cursor_position(0,1);
    lcd.print("Hello world!").unwrap();
   



    loop {
        
        //lcd.print("Hello world!").unwrap();
        arduino_hal::delay_ms(1000);
    }
}
