#[doc = "Register `INTR_SIE_SET` reader"]
pub type R = crate::R<IntrSieSetSpec>;
#[doc = "Register `INTR_SIE_SET` writer"]
pub type W = crate::W<IntrSieSetSpec>;
#[doc = "Field `SOF_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SofIntrSetR = crate::BitReader;
#[doc = "Field `SOF_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SofIntrSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type BusResetIntrSetR = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type BusResetIntrSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type Ep0IntrSetR = crate::BitReader;
#[doc = "Field `EP0_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type Ep0IntrSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type LpmIntrSetR = crate::BitReader;
#[doc = "Field `LPM_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type LpmIntrSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type ResumeIntrSetR = crate::BitReader;
#[doc = "Field `RESUME_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type ResumeIntrSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sof_intr_set(&self) -> SofIntrSetR {
        SofIntrSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn bus_reset_intr_set(&self) -> BusResetIntrSetR {
        BusResetIntrSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ep0_intr_set(&self) -> Ep0IntrSetR {
        Ep0IntrSetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn lpm_intr_set(&self) -> LpmIntrSetR {
        LpmIntrSetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn resume_intr_set(&self) -> ResumeIntrSetR {
        ResumeIntrSetR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn sof_intr_set(&mut self) -> SofIntrSetW<IntrSieSetSpec> {
        SofIntrSetW::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_intr_set(&mut self) -> BusResetIntrSetW<IntrSieSetSpec> {
        BusResetIntrSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ep0_intr_set(&mut self) -> Ep0IntrSetW<IntrSieSetSpec> {
        Ep0IntrSetW::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_intr_set(&mut self) -> LpmIntrSetW<IntrSieSetSpec> {
        LpmIntrSetW::new(self, 3)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn resume_intr_set(&mut self) -> ResumeIntrSetW<IntrSieSetSpec> {
        ResumeIntrSetW::new(self, 4)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sie_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSieSetSpec;
impl crate::RegisterSpec for IntrSieSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie_set::R`](R) reader structure"]
impl crate::Readable for IntrSieSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_sie_set::W`](W) writer structure"]
impl crate::Writable for IntrSieSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SIE_SET to value 0"]
impl crate::Resettable for IntrSieSetSpec {
    const RESET_VALUE: u32 = 0;
}
