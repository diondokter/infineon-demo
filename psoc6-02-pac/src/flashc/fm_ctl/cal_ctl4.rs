#[doc = "Register `CAL_CTL4` reader"]
pub type R = crate::R<CalCtl4Spec>;
#[doc = "Register `CAL_CTL4` writer"]
pub type W = crate::W<CalCtl4Spec>;
#[doc = "Field `VLIM_TRIM_ULP_HV` reader - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
pub type VlimTrimUlpHvR = crate::FieldReader;
#[doc = "Field `VLIM_TRIM_ULP_HV` writer - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
pub type VlimTrimUlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDAC_ULP_HV` reader - Sets the sense current reference offset value. Refer to trim tables for details."]
pub type IdacUlpHvR = crate::FieldReader;
#[doc = "Field `IDAC_ULP_HV` writer - Sets the sense current reference offset value. Refer to trim tables for details."]
pub type IdacUlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDAC_ULP_HV` reader - Sets the sense current reference temp slope. Refer to trim tables for details."]
pub type SdacUlpHvR = crate::FieldReader;
#[doc = "Field `SDAC_ULP_HV` writer - Sets the sense current reference temp slope. Refer to trim tables for details."]
pub type SdacUlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ITIM_ULP_HV` reader - Trimming of timing current"]
pub type ItimUlpHvR = crate::FieldReader;
#[doc = "Field `ITIM_ULP_HV` writer - Trimming of timing current"]
pub type ItimUlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FM_READY_DEL_ULP_HV` reader - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
pub type FmReadyDelUlpHvR = crate::FieldReader;
#[doc = "Field `FM_READY_DEL_ULP_HV` writer - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
pub type FmReadyDelUlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE451_ULP_HV` reader - N/A"]
pub type Spare451UlpHvR = crate::BitReader;
#[doc = "Field `SPARE451_ULP_HV` writer - N/A"]
pub type Spare451UlpHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY_RESTART_N_HV` reader - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
pub type ReadyRestartNHvR = crate::BitReader;
#[doc = "Field `READY_RESTART_N_HV` writer - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
pub type ReadyRestartNHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBST_S_DIS_HV` reader - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
pub type VbstSDisHvR = crate::BitReader;
#[doc = "Field `VBST_S_DIS_HV` writer - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
pub type VbstSDisHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_HVPULSE_HV` reader - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
pub type AutoHvpulseHvR = crate::BitReader;
#[doc = "Field `AUTO_HVPULSE_HV` writer - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
pub type AutoHvpulseHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UGB_EN_HV` reader - UGB enable in TM control"]
pub type UgbEnHvR = crate::BitReader;
#[doc = "Field `UGB_EN_HV` writer - UGB enable in TM control"]
pub type UgbEnHvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_ulp_hv(&self) -> VlimTrimUlpHvR {
        VlimTrimUlpHvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn idac_ulp_hv(&self) -> IdacUlpHvR {
        IdacUlpHvR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn sdac_ulp_hv(&self) -> SdacUlpHvR {
        SdacUlpHvR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_ulp_hv(&self) -> ItimUlpHvR {
        ItimUlpHvR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn fm_ready_del_ulp_hv(&self) -> FmReadyDelUlpHvR {
        FmReadyDelUlpHvR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn spare451_ulp_hv(&self) -> Spare451UlpHvR {
        Spare451UlpHvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    pub fn ready_restart_n_hv(&self) -> ReadyRestartNHvR {
        ReadyRestartNHvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    pub fn vbst_s_dis_hv(&self) -> VbstSDisHvR {
        VbstSDisHvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    pub fn auto_hvpulse_hv(&self) -> AutoHvpulseHvR {
        AutoHvpulseHvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UGB enable in TM control"]
    #[inline(always)]
    pub fn ugb_en_hv(&self) -> UgbEnHvR {
        UgbEnHvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    #[must_use]
    pub fn vlim_trim_ulp_hv(&mut self) -> VlimTrimUlpHvW<CalCtl4Spec> {
        VlimTrimUlpHvW::new(self, 0)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    #[must_use]
    pub fn idac_ulp_hv(&mut self) -> IdacUlpHvW<CalCtl4Spec> {
        IdacUlpHvW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    #[must_use]
    pub fn sdac_ulp_hv(&mut self) -> SdacUlpHvW<CalCtl4Spec> {
        SdacUlpHvW::new(self, 6)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    #[must_use]
    pub fn itim_ulp_hv(&mut self) -> ItimUlpHvW<CalCtl4Spec> {
        ItimUlpHvW::new(self, 8)
    }
    #[doc = "Bits 13:14 - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    #[must_use]
    pub fn fm_ready_del_ulp_hv(&mut self) -> FmReadyDelUlpHvW<CalCtl4Spec> {
        FmReadyDelUlpHvW::new(self, 13)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spare451_ulp_hv(&mut self) -> Spare451UlpHvW<CalCtl4Spec> {
        Spare451UlpHvW::new(self, 15)
    }
    #[doc = "Bit 16 - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    #[must_use]
    pub fn ready_restart_n_hv(&mut self) -> ReadyRestartNHvW<CalCtl4Spec> {
        ReadyRestartNHvW::new(self, 16)
    }
    #[doc = "Bit 17 - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    #[must_use]
    pub fn vbst_s_dis_hv(&mut self) -> VbstSDisHvW<CalCtl4Spec> {
        VbstSDisHvW::new(self, 17)
    }
    #[doc = "Bit 18 - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    #[must_use]
    pub fn auto_hvpulse_hv(&mut self) -> AutoHvpulseHvW<CalCtl4Spec> {
        AutoHvpulseHvW::new(self, 18)
    }
    #[doc = "Bit 19 - UGB enable in TM control"]
    #[inline(always)]
    #[must_use]
    pub fn ugb_en_hv(&mut self) -> UgbEnHvW<CalCtl4Spec> {
        UgbEnHvW::new(self, 19)
    }
}
#[doc = "Cal Control Vlim, SA, fdiv, reg_act\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtl4Spec;
impl crate::RegisterSpec for CalCtl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl4::R`](R) reader structure"]
impl crate::Readable for CalCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl4::W`](W) writer structure"]
impl crate::Writable for CalCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL4 to value 0x0001_2ae0"]
impl crate::Resettable for CalCtl4Spec {
    const RESET_VALUE: u32 = 0x0001_2ae0;
}
