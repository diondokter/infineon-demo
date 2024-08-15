#[doc = "Register `PWR_TRIM_REF_CTL` reader"]
pub type R = crate::R<PwrTrimRefCtlSpec>;
#[doc = "Register `PWR_TRIM_REF_CTL` writer"]
pub type W = crate::W<PwrTrimRefCtlSpec>;
#[doc = "Field `ACT_REF_TCTRIM` reader - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ActRefTctrimR = crate::FieldReader;
#[doc = "Field `ACT_REF_TCTRIM` writer - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ActRefTctrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACT_REF_ITRIM` reader - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ActRefItrimR = crate::FieldReader;
#[doc = "Field `ACT_REF_ITRIM` writer - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ActRefItrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACT_REF_ABSTRIM` reader - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ActRefAbstrimR = crate::FieldReader;
#[doc = "Field `ACT_REF_ABSTRIM` writer - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ActRefAbstrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ACT_REF_IBOOST` reader - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub type ActRefIboostR = crate::BitReader;
#[doc = "Field `ACT_REF_IBOOST` writer - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub type ActRefIboostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPSLP_REF_TCTRIM` reader - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type DpslpRefTctrimR = crate::FieldReader;
#[doc = "Field `DPSLP_REF_TCTRIM` writer - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type DpslpRefTctrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPSLP_REF_ABSTRIM` reader - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DpslpRefAbstrimR = crate::FieldReader;
#[doc = "Field `DPSLP_REF_ABSTRIM` writer - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DpslpRefAbstrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DPSLP_REF_ITRIM` reader - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DpslpRefItrimR = crate::FieldReader;
#[doc = "Field `DPSLP_REF_ITRIM` writer - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DpslpRefItrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&self) -> ActRefTctrimR {
        ActRefTctrimR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&self) -> ActRefItrimR {
        ActRefItrimR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&self) -> ActRefAbstrimR {
        ActRefAbstrimR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&self) -> ActRefIboostR {
        ActRefIboostR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&self) -> DpslpRefTctrimR {
        DpslpRefTctrimR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&self) -> DpslpRefAbstrimR {
        DpslpRefAbstrimR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&self) -> DpslpRefItrimR {
        DpslpRefItrimR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_tctrim(&mut self) -> ActRefTctrimW<PwrTrimRefCtlSpec> {
        ActRefTctrimW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_itrim(&mut self) -> ActRefItrimW<PwrTrimRefCtlSpec> {
        ActRefItrimW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_abstrim(&mut self) -> ActRefAbstrimW<PwrTrimRefCtlSpec> {
        ActRefAbstrimW::new(self, 8)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_iboost(&mut self) -> ActRefIboostW<PwrTrimRefCtlSpec> {
        ActRefIboostW::new(self, 14)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_ref_tctrim(&mut self) -> DpslpRefTctrimW<PwrTrimRefCtlSpec> {
        DpslpRefTctrimW::new(self, 16)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_ref_abstrim(&mut self) -> DpslpRefAbstrimW<PwrTrimRefCtlSpec> {
        DpslpRefAbstrimW::new(self, 20)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_ref_itrim(&mut self) -> DpslpRefItrimW<PwrTrimRefCtlSpec> {
        DpslpRefItrimW::new(self, 28)
    }
}
#[doc = "Reference Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_ref_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_ref_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrTrimRefCtlSpec;
impl crate::RegisterSpec for PwrTrimRefCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_ref_ctl::R`](R) reader structure"]
impl crate::Readable for PwrTrimRefCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_ref_ctl::W`](W) writer structure"]
impl crate::Writable for PwrTrimRefCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_REF_CTL to value 0x70f0_0000"]
impl crate::Resettable for PwrTrimRefCtlSpec {
    const RESET_VALUE: u32 = 0x70f0_0000;
}
