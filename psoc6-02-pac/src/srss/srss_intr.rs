#[doc = "Register `SRSS_INTR` reader"]
pub type R = crate::R<SrssIntrSpec>;
#[doc = "Register `SRSS_INTR` writer"]
pub type W = crate::W<SrssIntrSpec>;
#[doc = "Field `WDT_MATCH` reader - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
pub type WdtMatchR = crate::BitReader;
#[doc = "Field `WDT_MATCH` writer - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
pub type WdtMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVLVD1` reader - Interrupt for low voltage detector HVLVD1"]
pub type Hvlvd1R = crate::BitReader;
#[doc = "Field `HVLVD1` writer - Interrupt for low voltage detector HVLVD1"]
pub type Hvlvd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CAL` reader - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
pub type ClkCalR = crate::BitReader;
#[doc = "Field `CLK_CAL` writer - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
pub type ClkCalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WdtMatchR {
        WdtMatchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&self) -> Hvlvd1R {
        Hvlvd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub fn clk_cal(&self) -> ClkCalR {
        ClkCalR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match(&mut self) -> WdtMatchW<SrssIntrSpec> {
        WdtMatchW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1(&mut self) -> Hvlvd1W<SrssIntrSpec> {
        Hvlvd1W::new(self, 1)
    }
    #[doc = "Bit 5 - Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn clk_cal(&mut self) -> ClkCalW<SrssIntrSpec> {
        ClkCalW::new(self, 5)
    }
}
#[doc = "SRSS Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrssIntrSpec;
impl crate::RegisterSpec for SrssIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr::R`](R) reader structure"]
impl crate::Readable for SrssIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`srss_intr::W`](W) writer structure"]
impl crate::Writable for SrssIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSS_INTR to value 0"]
impl crate::Resettable for SrssIntrSpec {
    const RESET_VALUE: u32 = 0;
}
