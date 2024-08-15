#[doc = "Register `CAL_CTL3` reader"]
pub type R = crate::R<CalCtl3Spec>;
#[doc = "Register `CAL_CTL3` writer"]
pub type W = crate::W<CalCtl3Spec>;
#[doc = "Field `OSC_TRIM_HV` reader - Flash macro pump clock trim control."]
pub type OscTrimHvR = crate::FieldReader;
#[doc = "Field `OSC_TRIM_HV` writer - Flash macro pump clock trim control."]
pub type OscTrimHvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OSC_RANGE_TRIM_HV` reader - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub type OscRangeTrimHvR = crate::BitReader;
#[doc = "Field `OSC_RANGE_TRIM_HV` writer - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub type OscRangeTrimHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPROT_ACT_HV` reader - Forces VPROT in active mode all the time"]
pub type VprotActHvR = crate::BitReader;
#[doc = "Field `VPROT_ACT_HV` writer - Forces VPROT in active mode all the time"]
pub type VprotActHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPREF_TC_HV` reader - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
pub type IprefTcHvR = crate::BitReader;
#[doc = "Field `IPREF_TC_HV` writer - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
pub type IprefTcHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL_HV` reader - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
pub type VrefSelHvR = crate::BitReader;
#[doc = "Field `VREF_SEL_HV` writer - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
pub type VrefSelHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREF_SEL_HV` reader - Current reference: 0: internal current reference 1: external current reference"]
pub type IrefSelHvR = crate::BitReader;
#[doc = "Field `IREF_SEL_HV` writer - Current reference: 0: internal current reference 1: external current reference"]
pub type IrefSelHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ACT_HV` reader - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
pub type RegActHvR = crate::BitReader;
#[doc = "Field `REG_ACT_HV` writer - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
pub type RegActHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIV_TRIM_HV` reader - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
pub type FdivTrimHvR = crate::FieldReader;
#[doc = "Field `FDIV_TRIM_HV` writer - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
pub type FdivTrimHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDDHI_HV` reader - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
pub type VddhiHvR = crate::BitReader;
#[doc = "Field `VDDHI_HV` writer - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
pub type VddhiHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURBO_PULSEW_HV` reader - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
pub type TurboPulsewHvR = crate::FieldReader;
#[doc = "Field `TURBO_PULSEW_HV` writer - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
pub type TurboPulsewHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BGLO_EN_HV` reader - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
pub type BgloEnHvR = crate::BitReader;
#[doc = "Field `BGLO_EN_HV` writer - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
pub type BgloEnHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGHI_EN_HV` reader - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
pub type BghiEnHvR = crate::BitReader;
#[doc = "Field `BGHI_EN_HV` writer - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
pub type BghiEnHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL_ISO_DIS_HV` reader - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
pub type ClIsoDisHvR = crate::BitReader;
#[doc = "Field `CL_ISO_DIS_HV` writer - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
pub type ClIsoDisHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R_GRANT_EN_HV` reader - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
pub type RGrantEnHvR = crate::BitReader;
#[doc = "Field `R_GRANT_EN_HV` writer - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
pub type RGrantEnHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ULP_SW_HV` reader - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
pub type LpUlpSwHvR = crate::BitReader;
#[doc = "Field `LP_ULP_SW_HV` writer - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
pub type LpUlpSwHvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&self) -> OscTrimHvR {
        OscTrimHvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&self) -> OscRangeTrimHvR {
        OscRangeTrimHvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn vprot_act_hv(&self) -> VprotActHvR {
        VprotActHvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
    #[inline(always)]
    pub fn ipref_tc_hv(&self) -> IprefTcHvR {
        IprefTcHvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
    #[inline(always)]
    pub fn vref_sel_hv(&self) -> VrefSelHvR {
        VrefSelHvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Current reference: 0: internal current reference 1: external current reference"]
    #[inline(always)]
    pub fn iref_sel_hv(&self) -> IrefSelHvR {
        IrefSelHvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&self) -> RegActHvR {
        RegActHvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
    #[inline(always)]
    pub fn fdiv_trim_hv(&self) -> FdivTrimHvR {
        FdivTrimHvR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
    #[inline(always)]
    pub fn vddhi_hv(&self) -> VddhiHvR {
        VddhiHvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&self) -> TurboPulsewHvR {
        TurboPulsewHvR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
    #[inline(always)]
    pub fn bglo_en_hv(&self) -> BgloEnHvR {
        BgloEnHvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
    #[inline(always)]
    pub fn bghi_en_hv(&self) -> BghiEnHvR {
        BghiEnHvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
    #[inline(always)]
    pub fn cl_iso_dis_hv(&self) -> ClIsoDisHvR {
        ClIsoDisHvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
    #[inline(always)]
    pub fn r_grant_en_hv(&self) -> RGrantEnHvR {
        RGrantEnHvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
    #[inline(always)]
    pub fn lp_ulp_sw_hv(&self) -> LpUlpSwHvR {
        LpUlpSwHvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    #[must_use]
    pub fn osc_trim_hv(&mut self) -> OscTrimHvW<CalCtl3Spec> {
        OscTrimHvW::new(self, 0)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn osc_range_trim_hv(&mut self) -> OscRangeTrimHvW<CalCtl3Spec> {
        OscRangeTrimHvW::new(self, 4)
    }
    #[doc = "Bit 5 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    #[must_use]
    pub fn vprot_act_hv(&mut self) -> VprotActHvW<CalCtl3Spec> {
        VprotActHvW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
    #[inline(always)]
    #[must_use]
    pub fn ipref_tc_hv(&mut self) -> IprefTcHvW<CalCtl3Spec> {
        IprefTcHvW::new(self, 6)
    }
    #[doc = "Bit 7 - Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel_hv(&mut self) -> VrefSelHvW<CalCtl3Spec> {
        VrefSelHvW::new(self, 7)
    }
    #[doc = "Bit 8 - Current reference: 0: internal current reference 1: external current reference"]
    #[inline(always)]
    #[must_use]
    pub fn iref_sel_hv(&mut self) -> IrefSelHvW<CalCtl3Spec> {
        IrefSelHvW::new(self, 8)
    }
    #[doc = "Bit 9 - 0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    #[must_use]
    pub fn reg_act_hv(&mut self) -> RegActHvW<CalCtl3Spec> {
        RegActHvW::new(self, 9)
    }
    #[doc = "Bits 10:11 - FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
    #[inline(always)]
    #[must_use]
    pub fn fdiv_trim_hv(&mut self) -> FdivTrimHvW<CalCtl3Spec> {
        FdivTrimHvW::new(self, 10)
    }
    #[doc = "Bit 12 - 0: vdd &lt; 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
    #[inline(always)]
    #[must_use]
    pub fn vddhi_hv(&mut self) -> VddhiHvW<CalCtl3Spec> {
        VddhiHvW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
    #[inline(always)]
    #[must_use]
    pub fn turbo_pulsew_hv(&mut self) -> TurboPulsewHvW<CalCtl3Spec> {
        TurboPulsewHvW::new(self, 13)
    }
    #[doc = "Bit 15 - 0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
    #[inline(always)]
    #[must_use]
    pub fn bglo_en_hv(&mut self) -> BgloEnHvW<CalCtl3Spec> {
        BgloEnHvW::new(self, 15)
    }
    #[doc = "Bit 16 - 0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
    #[inline(always)]
    #[must_use]
    pub fn bghi_en_hv(&mut self) -> BghiEnHvW<CalCtl3Spec> {
        BghiEnHvW::new(self, 16)
    }
    #[doc = "Bit 17 - 0: The internal logic controls the CL isolation 1: Forces CL bypass"]
    #[inline(always)]
    #[must_use]
    pub fn cl_iso_dis_hv(&mut self) -> ClIsoDisHvW<CalCtl3Spec> {
        ClIsoDisHvW::new(self, 17)
    }
    #[doc = "Bit 18 - 0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
    #[inline(always)]
    #[must_use]
    pub fn r_grant_en_hv(&mut self) -> RGrantEnHvW<CalCtl3Spec> {
        RGrantEnHvW::new(self, 18)
    }
    #[doc = "Bit 19 - LP&lt;-->ULP switch for trim signals: 0: LP 1: ULP"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ulp_sw_hv(&mut self) -> LpUlpSwHvW<CalCtl3Spec> {
        LpUlpSwHvW::new(self, 19)
    }
}
#[doc = "Cal control osc trim bits, idac, sdac, itim\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtl3Spec;
impl crate::RegisterSpec for CalCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl3::R`](R) reader structure"]
impl crate::Readable for CalCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl3::W`](W) writer structure"]
impl crate::Writable for CalCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL3 to value 0x2004"]
impl crate::Resettable for CalCtl3Spec {
    const RESET_VALUE: u32 = 0x2004;
}
