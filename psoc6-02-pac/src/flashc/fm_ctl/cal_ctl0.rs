#[doc = "Register `CAL_CTL0` reader"]
pub type R = crate::R<CalCtl0Spec>;
#[doc = "Register `CAL_CTL0` writer"]
pub type W = crate::W<CalCtl0Spec>;
#[doc = "Field `VCT_TRIM_LO_HV` reader - LO Bandgap Voltage Temperature Compensation trim control."]
pub type VctTrimLoHvR = crate::FieldReader;
#[doc = "Field `VCT_TRIM_LO_HV` writer - LO Bandgap Voltage Temperature Compensation trim control."]
pub type VctTrimLoHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CDAC_LO_HV` reader - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CdacLoHvR = crate::FieldReader;
#[doc = "Field `CDAC_LO_HV` writer - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CdacLoHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VBG_TRIM_LO_HV` reader - LO Bandgap Voltage trim control."]
pub type VbgTrimLoHvR = crate::FieldReader;
#[doc = "Field `VBG_TRIM_LO_HV` writer - LO Bandgap Voltage trim control."]
pub type VbgTrimLoHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VBG_TC_TRIM_LO_HV` reader - LO Bandgap Voltage Temperature Compensation trim control"]
pub type VbgTcTrimLoHvR = crate::FieldReader;
#[doc = "Field `VBG_TC_TRIM_LO_HV` writer - LO Bandgap Voltage Temperature Compensation trim control"]
pub type VbgTcTrimLoHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICREF_TC_TRIM_LO_HV` reader - LO Bandgap Current Temperature Compensation trim control"]
pub type IcrefTcTrimLoHvR = crate::FieldReader;
#[doc = "Field `ICREF_TC_TRIM_LO_HV` writer - LO Bandgap Current Temperature Compensation trim control"]
pub type IcrefTcTrimLoHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IPREF_TRIMA_LO_HV` reader - Adds 100-150nA boost on IPREF_LO"]
pub type IprefTrimaLoHvR = crate::BitReader;
#[doc = "Field `IPREF_TRIMA_LO_HV` writer - Adds 100-150nA boost on IPREF_LO"]
pub type IprefTrimaLoHvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vct_trim_lo_hv(&self) -> VctTrimLoHvR {
        VctTrimLoHvR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn cdac_lo_hv(&self) -> CdacLoHvR {
        CdacLoHvR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - LO Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn vbg_trim_lo_hv(&self) -> VbgTrimLoHvR {
        VbgTrimLoHvR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    pub fn vbg_tc_trim_lo_hv(&self) -> VbgTcTrimLoHvR {
        VbgTcTrimLoHvR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&self) -> IcrefTcTrimLoHvR {
        IcrefTcTrimLoHvR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&self) -> IprefTrimaLoHvR {
        IprefTrimaLoHvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    #[must_use]
    pub fn vct_trim_lo_hv(&mut self) -> VctTrimLoHvW<CalCtl0Spec> {
        VctTrimLoHvW::new(self, 0)
    }
    #[doc = "Bits 5:7 - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    #[must_use]
    pub fn cdac_lo_hv(&mut self) -> CdacLoHvW<CalCtl0Spec> {
        CdacLoHvW::new(self, 5)
    }
    #[doc = "Bits 8:12 - LO Bandgap Voltage trim control."]
    #[inline(always)]
    #[must_use]
    pub fn vbg_trim_lo_hv(&mut self) -> VbgTrimLoHvW<CalCtl0Spec> {
        VbgTrimLoHvW::new(self, 8)
    }
    #[doc = "Bits 13:15 - LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_tc_trim_lo_hv(&mut self) -> VbgTcTrimLoHvW<CalCtl0Spec> {
        VbgTcTrimLoHvW::new(self, 13)
    }
    #[doc = "Bits 16:18 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    #[must_use]
    pub fn icref_tc_trim_lo_hv(&mut self) -> IcrefTcTrimLoHvW<CalCtl0Spec> {
        IcrefTcTrimLoHvW::new(self, 16)
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trima_lo_hv(&mut self) -> IprefTrimaLoHvW<CalCtl0Spec> {
        IprefTrimaLoHvW::new(self, 19)
    }
}
#[doc = "Cal control BG LO trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtl0Spec;
impl crate::RegisterSpec for CalCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl0::R`](R) reader structure"]
impl crate::Readable for CalCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl0::W`](W) writer structure"]
impl crate::Writable for CalCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL0 to value 0x0003_8f8f"]
impl crate::Resettable for CalCtl0Spec {
    const RESET_VALUE: u32 = 0x0003_8f8f;
}
