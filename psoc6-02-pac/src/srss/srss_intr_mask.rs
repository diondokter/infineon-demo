#[doc = "Register `SRSS_INTR_MASK` reader"]
pub type R = crate::R<SrssIntrMaskSpec>;
#[doc = "Register `SRSS_INTR_MASK` writer"]
pub type W = crate::W<SrssIntrMaskSpec>;
#[doc = "Field `WDT_MATCH` reader - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
pub type WdtMatchR = crate::BitReader;
#[doc = "Field `WDT_MATCH` writer - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
pub type WdtMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVLVD1` reader - Mask for low voltage detector HVLVD1"]
pub type Hvlvd1R = crate::BitReader;
#[doc = "Field `HVLVD1` writer - Mask for low voltage detector HVLVD1"]
pub type Hvlvd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CAL` reader - Mask for clock calibration done"]
pub type ClkCalR = crate::BitReader;
#[doc = "Field `CLK_CAL` writer - Mask for clock calibration done"]
pub type ClkCalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WdtMatchR {
        WdtMatchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&self) -> Hvlvd1R {
        Hvlvd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask for clock calibration done"]
    #[inline(always)]
    pub fn clk_cal(&self) -> ClkCalR {
        ClkCalR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match(&mut self) -> WdtMatchW<SrssIntrMaskSpec> {
        WdtMatchW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1(&mut self) -> Hvlvd1W<SrssIntrMaskSpec> {
        Hvlvd1W::new(self, 1)
    }
    #[doc = "Bit 5 - Mask for clock calibration done"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cal(&mut self) -> ClkCalW<SrssIntrMaskSpec> {
        ClkCalW::new(self, 5)
    }
}
#[doc = "SRSS Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrssIntrMaskSpec;
impl crate::RegisterSpec for SrssIntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr_mask::R`](R) reader structure"]
impl crate::Readable for SrssIntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`srss_intr_mask::W`](W) writer structure"]
impl crate::Writable for SrssIntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSS_INTR_MASK to value 0"]
impl crate::Resettable for SrssIntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
