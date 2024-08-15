#[doc = "Register `CAL_CTL` reader"]
pub type R = crate::R<CalCtlSpec>;
#[doc = "Register `CAL_CTL` writer"]
pub type W = crate::W<CalCtlSpec>;
#[doc = "Field `CALIB_VAL` reader - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
pub type CalibValR = crate::FieldReader;
#[doc = "Field `CALIB_VAL` writer - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
pub type CalibValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CALIB_SIGN` reader - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CalibSignR = crate::BitReader;
#[doc = "Field `CALIB_SIGN` writer - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CalibSignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_OUT` reader - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
pub type CalOutR = crate::BitReader;
#[doc = "Field `CAL_OUT` writer - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
pub type CalOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub fn calib_val(&self) -> CalibValR {
        CalibValR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn calib_sign(&self) -> CalibSignR {
        CalibSignR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub fn cal_out(&self) -> CalOutR {
        CalOutR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    #[must_use]
    pub fn calib_val(&mut self) -> CalibValW<CalCtlSpec> {
        CalibValW::new(self, 0)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    #[must_use]
    pub fn calib_sign(&mut self) -> CalibSignW<CalCtlSpec> {
        CalibSignW::new(self, 6)
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    #[must_use]
    pub fn cal_out(&mut self) -> CalOutW<CalCtlSpec> {
        CalOutW::new(self, 31)
    }
}
#[doc = "Oscillator calibration for absolute frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtlSpec;
impl crate::RegisterSpec for CalCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl::R`](R) reader structure"]
impl crate::Readable for CalCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl::W`](W) writer structure"]
impl crate::Writable for CalCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL to value 0"]
impl crate::Resettable for CalCtlSpec {
    const RESET_VALUE: u32 = 0;
}
