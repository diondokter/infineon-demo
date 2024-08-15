#[doc = "Register `ROM_CTL` reader"]
pub type R = crate::R<RomCtlSpec>;
#[doc = "Register `ROM_CTL` writer"]
pub type W = crate::W<RomCtlSpec>;
#[doc = "Field `SLOW_WS` reader - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. ROM_CTL.SLOW_WS = '0' when clk_hf &lt;=100 MHz. ROM_CTL.SLOW_WS = '1' when 100MHz &lt; clk_hf &lt;=clk_hf_max. Note: clk_hf_max depends on the target device. Refer datasheet."]
pub type SlowWsR = crate::FieldReader;
#[doc = "Field `SLOW_WS` writer - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. ROM_CTL.SLOW_WS = '0' when clk_hf &lt;=100 MHz. ROM_CTL.SLOW_WS = '1' when 100MHz &lt; clk_hf &lt;=clk_hf_max. Note: clk_hf_max depends on the target device. Refer datasheet."]
pub type SlowWsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAST_WS` reader - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles. ROM_CTL.FAST_WS = '0' when clk_hf &lt;= clk_hf_max."]
pub type FastWsR = crate::FieldReader;
#[doc = "Field `FAST_WS` writer - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles. ROM_CTL.FAST_WS = '0' when clk_hf &lt;= clk_hf_max."]
pub type FastWsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. ROM_CTL.SLOW_WS = '0' when clk_hf &lt;=100 MHz. ROM_CTL.SLOW_WS = '1' when 100MHz &lt; clk_hf &lt;=clk_hf_max. Note: clk_hf_max depends on the target device. Refer datasheet."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SlowWsR {
        SlowWsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles. ROM_CTL.FAST_WS = '0' when clk_hf &lt;= clk_hf_max."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FastWsR {
        FastWsR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. ROM_CTL.SLOW_WS = '0' when clk_hf &lt;=100 MHz. ROM_CTL.SLOW_WS = '1' when 100MHz &lt; clk_hf &lt;=clk_hf_max. Note: clk_hf_max depends on the target device. Refer datasheet."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SlowWsW<RomCtlSpec> {
        SlowWsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles. ROM_CTL.FAST_WS = '0' when clk_hf &lt;= clk_hf_max."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FastWsW<RomCtlSpec> {
        FastWsW::new(self, 8)
    }
}
#[doc = "ROM control\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomCtlSpec;
impl crate::RegisterSpec for RomCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_ctl::R`](R) reader structure"]
impl crate::Readable for RomCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_ctl::W`](W) writer structure"]
impl crate::Writable for RomCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROM_CTL to value 0x01"]
impl crate::Resettable for RomCtlSpec {
    const RESET_VALUE: u32 = 0x01;
}
