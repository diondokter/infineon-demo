#[doc = "Register `INTR_SIE_MASK` reader"]
pub type R = crate::R<IntrSieMaskSpec>;
#[doc = "Register `INTR_SIE_MASK` writer"]
pub type W = crate::W<IntrSieMaskSpec>;
#[doc = "Field `SOF_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type SofIntrMaskR = crate::BitReader;
#[doc = "Field `SOF_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type SofIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type BusResetIntrMaskR = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type BusResetIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type Ep0IntrMaskR = crate::BitReader;
#[doc = "Field `EP0_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type Ep0IntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type LpmIntrMaskR = crate::BitReader;
#[doc = "Field `LPM_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type LpmIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type ResumeIntrMaskR = crate::BitReader;
#[doc = "Field `RESUME_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub type ResumeIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn sof_intr_mask(&self) -> SofIntrMaskR {
        SofIntrMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn bus_reset_intr_mask(&self) -> BusResetIntrMaskR {
        BusResetIntrMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn ep0_intr_mask(&self) -> Ep0IntrMaskR {
        Ep0IntrMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn lpm_intr_mask(&self) -> LpmIntrMaskR {
        LpmIntrMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn resume_intr_mask(&self) -> ResumeIntrMaskR {
        ResumeIntrMaskR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn sof_intr_mask(&mut self) -> SofIntrMaskW<IntrSieMaskSpec> {
        SofIntrMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_intr_mask(&mut self) -> BusResetIntrMaskW<IntrSieMaskSpec> {
        BusResetIntrMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_intr_mask(&mut self) -> Ep0IntrMaskW<IntrSieMaskSpec> {
        Ep0IntrMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_intr_mask(&mut self) -> LpmIntrMaskW<IntrSieMaskSpec> {
        LpmIntrMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    #[must_use]
    pub fn resume_intr_mask(&mut self) -> ResumeIntrMaskW<IntrSieMaskSpec> {
        ResumeIntrMaskW::new(self, 4)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sie_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSieMaskSpec;
impl crate::RegisterSpec for IntrSieMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie_mask::R`](R) reader structure"]
impl crate::Readable for IntrSieMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_sie_mask::W`](W) writer structure"]
impl crate::Writable for IntrSieMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SIE_MASK to value 0"]
impl crate::Resettable for IntrSieMaskSpec {
    const RESET_VALUE: u32 = 0;
}
