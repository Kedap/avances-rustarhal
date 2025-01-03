use core::marker::PhantomData;

use arduino_hal::{
    adc::{AdcChannel, AdcOps},
    hal::Atmega,
    port::{mode, Pin},
    Pins,
};
use panic_halt as _;

struct Sensores<PinAnalogico, OpsAdc>
where
    OpsAdc: AdcOps<Atmega>,
    PinAnalogico: AdcChannel<Atmega, OpsAdc>,
{
    phantom: PhantomData<OpsAdc>,
    adc: arduino_hal::Adc,
    led: Pin<mode::Output>,
    s0: Pin<mode::Output>,
    s1: Pin<mode::Output>,
    s2: Pin<mode::Output>,
    s3: Pin<mode::Output>,
    lector: PinAnalogico,
}

impl<PinAnalogico, OpsAdc> Sensores<PinAnalogico, OpsAdc>
where
    OpsAdc: AdcOps<Atmega>,
    PinAnalogico: AdcChannel<Atmega, OpsAdc>,
{
    fn new(
        adc: arduino_hal::Adc,
        led: Pin<mode::Output>,
        s0: Pin<mode::Output>,
        s1: Pin<mode::Output>,
        s2: Pin<mode::Output>,
        s3: Pin<mode::Output>,
        lector: PinAnalogico,
    ) -> Self {
        Self {
            phantom: PhantomData,
            adc,
            led,
            s0,
            s1,
            s2,
            s3,
            lector,
        }
    }

    //fn new_default(pins: Pins) -> Self {
    //    Self {
    //        phantom: PhantomData,
    //        adc: arduino_hal::Adc::new(dp.ADC, Default::default()),
    //        led: pins.d11.into_output(),
    //        s0: pins.a0.into_output(),
    //        s1: pins.a1.into_output(),
    //        s2: a2.into_output(),
    //        s3: a3.into_output(),
    //        lector: arduino_hal::adc::channel::ADC6,
    //    }
    //}
}
