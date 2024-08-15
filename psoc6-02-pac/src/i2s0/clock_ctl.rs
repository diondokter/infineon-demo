#[doc = "Register `CLOCK_CTL` reader"]
pub type R = crate::R<ClockCtlSpec>;
#[doc = "Register `CLOCK_CTL` writer"]
pub type W = crate::W<ClockCtlSpec>;
#[doc = "Field `CLOCK_DIV` reader - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub type ClockDivR = crate::FieldReader;
#[doc = "Field `CLOCK_DIV` writer - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub type ClockDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLOCK_SEL` reader - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub type ClockSelR = crate::BitReader;
#[doc = "Field `CLOCK_SEL` writer - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub type ClockSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn clock_div(&self) -> ClockDivR {
        ClockDivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn clock_sel(&self) -> ClockSelR {
        ClockSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    #[must_use]
    pub fn clock_div(&mut self) -> ClockDivW<ClockCtlSpec> {
        ClockDivW::new(self, 0)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    #[must_use]
    pub fn clock_sel(&mut self) -> ClockSelW<ClockCtlSpec> {
        ClockSelW::new(self, 8)
    }
}
#[doc = "Clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockCtlSpec;
impl crate::RegisterSpec for ClockCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_ctl::R`](R) reader structure"]
impl crate::Readable for ClockCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_ctl::W`](W) writer structure"]
impl crate::Writable for ClockCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0"]
impl crate::Resettable for ClockCtlSpec {
    const RESET_VALUE: u32 = 0;
}
