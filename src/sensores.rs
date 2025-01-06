use arduino_hal::port::{mode, Pin, PinOps};
use panic_halt as _;
use ufmt::{uDebug, uwrite};

use crate::comun::ToBin;

pub struct Sensores<PinAnalogico, T, U, V, W, X>
where
    PinAnalogico: arduino_hal::adc::AdcChannel<arduino_hal::hal::Atmega, arduino_hal::pac::ADC>,
    T: PinOps,
    U: PinOps,
    V: PinOps,
    W: PinOps,
    X: PinOps,
{
    adc: arduino_hal::Adc,
    led: Pin<mode::Output, T>,
    s0: Pin<mode::Output, U>,
    s1: Pin<mode::Output, V>,
    s2: Pin<mode::Output, W>,
    s3: Pin<mode::Output, X>,
    lector: PinAnalogico,
    valores: [u16; 16],
}

impl<PinAnalogico, T, U, V, W, X> Sensores<PinAnalogico, T, U, V, W, X>
where
    PinAnalogico: arduino_hal::adc::AdcChannel<arduino_hal::hal::Atmega, arduino_hal::pac::ADC>,
    T: PinOps,
    U: PinOps,
    V: PinOps,
    W: PinOps,
    X: PinOps,
{
    pub fn new(
        adc: arduino_hal::Adc,
        led: Pin<mode::Output, T>,
        s0: Pin<mode::Output, U>,
        s1: Pin<mode::Output, V>,
        s2: Pin<mode::Output, W>,
        s3: Pin<mode::Output, X>,
        lector: PinAnalogico,
    ) -> Self {
        let mut s = Self {
            adc,
            led,
            s0,
            s1,
            s2,
            s3,
            lector,
            valores: [0; 16],
        };
        s.s0.set_low();
        s.s1.set_low();
        s.s2.set_low();
        s.s3.set_low();
        s
    }

    pub fn actualizar_valores(&mut self) {
        if !self.led.is_set_high() {
            self.led.set_high();
        }
        (0..16).for_each(|i| {
            let digital = (i as u8).to_bin();
            if digital[0] == 0 {
                self.s0.set_low();
            } else {
                self.s0.set_high();
            }
            if digital[1] == 0 {
                self.s1.set_low();
            } else {
                self.s1.set_high();
            }
            if digital[2] == 0 {
                self.s2.set_low();
            } else {
                self.s2.set_high();
            }
            if digital[3] == 0 {
                self.s3.set_low();
            } else {
                self.s3.set_high();
            }
            self.valores[i] = self.adc.read_blocking(&self.lector);
        });
    }
}

impl<PinAnalogico, T, U, V, WW, X> uDebug for Sensores<PinAnalogico, T, U, V, WW, X>
where
    PinAnalogico: arduino_hal::adc::AdcChannel<arduino_hal::hal::Atmega, arduino_hal::pac::ADC>,
    T: PinOps,
    U: PinOps,
    V: PinOps,
    WW: PinOps,
    X: PinOps,
{
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        uwrite!(f, "{:?}", self.valores)
    }
}

#[macro_export]
macro_rules! sensores_default {
    ($dp:ident, $pins:ident) => {
        Sensores::new(
            arduino_hal::Adc::new($dp.ADC, Default::default()),
            $pins.d11.into_output(),
            $pins.a0.into_output(),
            $pins.a1.into_output(),
            $pins.a2.into_output(),
            $pins.a3.into_output(),
            arduino_hal::adc::channel::ADC6,
        )
    };
}
