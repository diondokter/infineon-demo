#[doc = "Register `CAL_CTL1` reader"]
pub type R = crate::R<CalCtl1Spec>;
#[doc = "Register `CAL_CTL1` writer"]
pub type W = crate::W<CalCtl1Spec>;
#[doc = "Field `VCT_TRIM_HI_HV` reader - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VctTrimHiHvR = crate::FieldReader;
#[doc = "Field `VCT_TRIM_HI_HV` writer - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VctTrimHiHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CDAC_HI_HV` reader - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CdacHiHvR = crate::FieldReader;
#[doc = "Field `CDAC_HI_HV` writer - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CdacHiHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VBG_TRIM_HI_HV` reader - HI Bandgap Voltage trim control."]
pub type VbgTrimHiHvR = crate::FieldReader;
#[doc = "Field `VBG_TRIM_HI_HV` writer - HI Bandgap Voltage trim control."]
pub type VbgTrimHiHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VBG_TC_TRIM_HI_HV` reader - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VbgTcTrimHiHvR = crate::FieldReader;
#[doc = "Field `VBG_TC_TRIM_HI_HV` writer - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VbgTcTrimHiHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICREF_TC_TRIM_HI_HV` reader - HI Bandgap Current Temperature Compensation trim control."]
pub type IcrefTcTrimHiHvR = crate::FieldReader;
#[doc = "Field `ICREF_TC_TRIM_HI_HV` writer - HI Bandgap Current Temperature Compensation trim control."]
pub type IcrefTcTrimHiHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IPREF_TRIMA_HI_HV` reader - Adds 100-150nA boost on IPREF_HI"]
pub type IprefTrimaHiHvR = crate::BitReader;
#[doc = "Field `IPREF_TRIMA_HI_HV` writer - Adds 100-150nA boost on IPREF_HI"]
pub type IprefTrimaHiHvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vct_trim_hi_hv(&self) -> VctTrimHiHvR {
        VctTrimHiHvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn cdac_hi_hv(&self) -> CdacHiHvR {
        CdacHiHvR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - HI Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn vbg_trim_hi_hv(&self) -> VbgTrimHiHvR {
        VbgTrimHiHvR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vbg_tc_trim_hi_hv(&self) -> VbgTcTrimHiHvR {
        VbgTcTrimHiHvR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub fn icref_tc_trim_hi_hv(&self) -> IcrefTcTrimHiHvR {
        IcrefTcTrimHiHvR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_HI"]
    #[inline(always)]
    pub fn ipref_trima_hi_hv(&self) -> IprefTrimaHiHvR {
        IprefTrimaHiHvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    #[must_use]
    pub fn vct_trim_hi_hv(&mut self) -> VctTrimHiHvW<CalCtl1Spec> {
        VctTrimHiHvW::new(self, 0)
    }
    #[doc = "Bits 5:7 - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    #[must_use]
    pub fn cdac_hi_hv(&mut self) -> CdacHiHvW<CalCtl1Spec> {
        CdacHiHvW::new(self, 5)
    }
    #[doc = "Bits 8:12 - HI Bandgap Voltage trim control."]
    #[inline(always)]
    #[must_use]
    pub fn vbg_trim_hi_hv(&mut self) -> VbgTrimHiHvW<CalCtl1Spec> {
        VbgTrimHiHvW::new(self, 8)
    }
    #[doc = "Bits 13:15 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    #[must_use]
    pub fn vbg_tc_trim_hi_hv(&mut self) -> VbgTcTrimHiHvW<CalCtl1Spec> {
        VbgTcTrimHiHvW::new(self, 13)
    }
    #[doc = "Bits 16:18 - HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    #[must_use]
    pub fn icref_tc_trim_hi_hv(&mut self) -> IcrefTcTrimHiHvW<CalCtl1Spec> {
        IcrefTcTrimHiHvW::new(self, 16)
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_HI"]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trima_hi_hv(&mut self) -> IprefTrimaHiHvW<CalCtl1Spec> {
        IprefTrimaHiHvW::new(self, 19)
    }
}
#[doc = "Cal control BG HI trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtl1Spec;
impl crate::RegisterSpec for CalCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl1::R`](R) reader structure"]
impl crate::Readable for CalCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl1::W`](W) writer structure"]
impl crate::Writable for CalCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL1 to value 0x0003_8f8f"]
impl crate::Resettable for CalCtl1Spec {
    const RESET_VALUE: u32 = 0x0003_8f8f;
}
