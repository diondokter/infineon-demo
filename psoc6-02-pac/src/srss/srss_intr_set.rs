#[doc = "Register `SRSS_INTR_SET` reader"]
pub type R = crate::R<SrssIntrSetSpec>;
#[doc = "Register `SRSS_INTR_SET` writer"]
pub type W = crate::W<SrssIntrSetSpec>;
#[doc = "Field `WDT_MATCH` reader - Set interrupt for low voltage detector WDT_MATCH"]
pub type WdtMatchR = crate::BitReader;
#[doc = "Field `WDT_MATCH` writer - Set interrupt for low voltage detector WDT_MATCH"]
pub type WdtMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVLVD1` reader - Set interrupt for low voltage detector HVLVD1"]
pub type Hvlvd1R = crate::BitReader;
#[doc = "Field `HVLVD1` writer - Set interrupt for low voltage detector HVLVD1"]
pub type Hvlvd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CAL` reader - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
pub type ClkCalR = crate::BitReader;
#[doc = "Field `CLK_CAL` writer - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
pub type ClkCalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set interrupt for low voltage detector WDT_MATCH"]
    #[inline(always)]
    pub fn wdt_match(&self) -> WdtMatchR {
        WdtMatchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&self) -> Hvlvd1R {
        Hvlvd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub fn clk_cal(&self) -> ClkCalR {
        ClkCalR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set interrupt for low voltage detector WDT_MATCH"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match(&mut self) -> WdtMatchW<SrssIntrSetSpec> {
        WdtMatchW::new(self, 0)
    }
    #[doc = "Bit 1 - Set interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1(&mut self) -> Hvlvd1W<SrssIntrSetSpec> {
        Hvlvd1W::new(self, 1)
    }
    #[doc = "Bit 5 - Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn clk_cal(&mut self) -> ClkCalW<SrssIntrSetSpec> {
        ClkCalW::new(self, 5)
    }
}
#[doc = "SRSS Interrupt Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrssIntrSetSpec;
impl crate::RegisterSpec for SrssIntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr_set::R`](R) reader structure"]
impl crate::Readable for SrssIntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`srss_intr_set::W`](W) writer structure"]
impl crate::Writable for SrssIntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSS_INTR_SET to value 0"]
impl crate::Resettable for SrssIntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
