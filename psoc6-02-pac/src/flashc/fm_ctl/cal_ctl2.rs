#[doc = "Register `CAL_CTL2` reader"]
pub type R = crate::R<CalCtl2Spec>;
#[doc = "Register `CAL_CTL2` writer"]
pub type W = crate::W<CalCtl2Spec>;
#[doc = "Field `ICREF_TRIM_LO_HV` reader - LO Bandgap Current trim control."]
pub type IcrefTrimLoHvR = crate::FieldReader;
#[doc = "Field `ICREF_TRIM_LO_HV` writer - LO Bandgap Current trim control."]
pub type IcrefTrimLoHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ICREF_TRIM_HI_HV` reader - HI Bandgap Current trim control."]
pub type IcrefTrimHiHvR = crate::FieldReader;
#[doc = "Field `ICREF_TRIM_HI_HV` writer - HI Bandgap Current trim control."]
pub type IcrefTrimHiHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IPREF_TRIM_LO_HV` reader - LO Bandgap IPTAT trim control."]
pub type IprefTrimLoHvR = crate::FieldReader;
#[doc = "Field `IPREF_TRIM_LO_HV` writer - LO Bandgap IPTAT trim control."]
pub type IprefTrimLoHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IPREF_TRIM_HI_HV` reader - HI Bandgap IPTAT trim control."]
pub type IprefTrimHiHvR = crate::FieldReader;
#[doc = "Field `IPREF_TRIM_HI_HV` writer - HI Bandgap IPTAT trim control."]
pub type IprefTrimHiHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&self) -> IcrefTrimLoHvR {
        IcrefTrimLoHvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&self) -> IcrefTrimHiHvR {
        IcrefTrimHiHvR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_lo_hv(&self) -> IprefTrimLoHvR {
        IprefTrimLoHvR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_hi_hv(&self) -> IprefTrimHiHvR {
        IprefTrimHiHvR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    #[must_use]
    pub fn icref_trim_lo_hv(&mut self) -> IcrefTrimLoHvW<CalCtl2Spec> {
        IcrefTrimLoHvW::new(self, 0)
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    #[must_use]
    pub fn icref_trim_hi_hv(&mut self) -> IcrefTrimHiHvW<CalCtl2Spec> {
        IcrefTrimHiHvW::new(self, 5)
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trim_lo_hv(&mut self) -> IprefTrimLoHvW<CalCtl2Spec> {
        IprefTrimLoHvW::new(self, 10)
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trim_hi_hv(&mut self) -> IprefTrimHiHvW<CalCtl2Spec> {
        IprefTrimHiHvW::new(self, 15)
    }
}
#[doc = "Cal control BG LO&amp;HI trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtl2Spec;
impl crate::RegisterSpec for CalCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl2::R`](R) reader structure"]
impl crate::Readable for CalCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl2::W`](W) writer structure"]
impl crate::Writable for CalCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL2 to value 0x0007_be10"]
impl crate::Resettable for CalCtl2Spec {
    const RESET_VALUE: u32 = 0x0007_be10;
}
