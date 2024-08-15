#[doc = "Register `INTR_SIE` reader"]
pub type R = crate::R<IntrSieSpec>;
#[doc = "Register `INTR_SIE` writer"]
pub type W = crate::W<IntrSieSpec>;
#[doc = "Field `SOF_INTR` reader - Interrupt status for USB SOF"]
pub type SofIntrR = crate::BitReader;
#[doc = "Field `SOF_INTR` writer - Interrupt status for USB SOF"]
pub type SofIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET_INTR` reader - Interrupt status for BUS RESET"]
pub type BusResetIntrR = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR` writer - Interrupt status for BUS RESET"]
pub type BusResetIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INTR` reader - Interrupt status for EP0"]
pub type Ep0IntrR = crate::BitReader;
#[doc = "Field `EP0_INTR` writer - Interrupt status for EP0"]
pub type Ep0IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_INTR` reader - Interrupt status for LPM (Link Power Management, L1 entry)"]
pub type LpmIntrR = crate::BitReader;
#[doc = "Field `LPM_INTR` writer - Interrupt status for LPM (Link Power Management, L1 entry)"]
pub type LpmIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_INTR` reader - Interrupt status for Resume"]
pub type ResumeIntrR = crate::BitReader;
#[doc = "Field `RESUME_INTR` writer - Interrupt status for Resume"]
pub type ResumeIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    pub fn sof_intr(&self) -> SofIntrR {
        SofIntrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    pub fn bus_reset_intr(&self) -> BusResetIntrR {
        BusResetIntrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    pub fn ep0_intr(&self) -> Ep0IntrR {
        Ep0IntrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub fn lpm_intr(&self) -> LpmIntrR {
        LpmIntrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    pub fn resume_intr(&self) -> ResumeIntrR {
        ResumeIntrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof_intr(&mut self) -> SofIntrW<IntrSieSpec> {
        SofIntrW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_intr(&mut self) -> BusResetIntrW<IntrSieSpec> {
        BusResetIntrW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_intr(&mut self) -> Ep0IntrW<IntrSieSpec> {
        Ep0IntrW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_intr(&mut self) -> LpmIntrW<IntrSieSpec> {
        LpmIntrW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    #[must_use]
    pub fn resume_intr(&mut self) -> ResumeIntrW<IntrSieSpec> {
        ResumeIntrW::new(self, 4)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSieSpec;
impl crate::RegisterSpec for IntrSieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie::R`](R) reader structure"]
impl crate::Readable for IntrSieSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_sie::W`](W) writer structure"]
impl crate::Writable for IntrSieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SIE to value 0"]
impl crate::Resettable for IntrSieSpec {
    const RESET_VALUE: u32 = 0;
}
