#[doc = "Register `CLK_TRIM_CCO_CTL` reader"]
pub type R = crate::R<ClkTrimCcoCtlSpec>;
#[doc = "Register `CLK_TRIM_CCO_CTL` writer"]
pub type W = crate::W<ClkTrimCcoCtlSpec>;
#[doc = "Field `CCO_RCSTRIM` reader - CCO reference current source trim."]
pub type CcoRcstrimR = crate::FieldReader;
#[doc = "Field `CCO_RCSTRIM` writer - CCO reference current source trim."]
pub type CcoRcstrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CCO_STABLE_CNT` reader - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub type CcoStableCntR = crate::FieldReader;
#[doc = "Field `CCO_STABLE_CNT` writer - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub type CcoStableCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ENABLE_CNT` reader - Enables the automatic stabilization counter."]
pub type EnableCntR = crate::BitReader;
#[doc = "Field `ENABLE_CNT` writer - Enables the automatic stabilization counter."]
pub type EnableCntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&self) -> CcoRcstrimR {
        CcoRcstrimR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&self) -> CcoStableCntR {
        CcoStableCntR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&self) -> EnableCntR {
        EnableCntR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    #[must_use]
    pub fn cco_rcstrim(&mut self) -> CcoRcstrimW<ClkTrimCcoCtlSpec> {
        CcoRcstrimW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    #[must_use]
    pub fn cco_stable_cnt(&mut self) -> CcoStableCntW<ClkTrimCcoCtlSpec> {
        CcoStableCntW::new(self, 24)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    #[must_use]
    pub fn enable_cnt(&mut self) -> EnableCntW<ClkTrimCcoCtlSpec> {
        EnableCntW::new(self, 31)
    }
}
#[doc = "CCO Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_cco_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_cco_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTrimCcoCtlSpec;
impl crate::RegisterSpec for ClkTrimCcoCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_cco_ctl::R`](R) reader structure"]
impl crate::Readable for ClkTrimCcoCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_cco_ctl::W`](W) writer structure"]
impl crate::Writable for ClkTrimCcoCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_CCO_CTL to value 0xa700_0020"]
impl crate::Resettable for ClkTrimCcoCtlSpec {
    const RESET_VALUE: u32 = 0xa700_0020;
}
