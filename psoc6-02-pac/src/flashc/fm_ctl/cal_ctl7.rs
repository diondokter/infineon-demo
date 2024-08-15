#[doc = "Register `CAL_CTL7` reader"]
pub type R = crate::R<CalCtl7Spec>;
#[doc = "Register `CAL_CTL7` writer"]
pub type W = crate::W<CalCtl7Spec>;
#[doc = "Field `ERSX8_CLK_SEL_HV` reader - Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
pub type Ersx8ClkSelHvR = crate::FieldReader;
#[doc = "Field `ERSX8_CLK_SEL_HV` writer - Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
pub type Ersx8ClkSelHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FM_ACTIVE_HV` reader - 0: Normal operation 1: Forces FM SYS in active mode"]
pub type FmActiveHvR = crate::BitReader;
#[doc = "Field `FM_ACTIVE_HV` writer - 0: Normal operation 1: Forces FM SYS in active mode"]
pub type FmActiveHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURBO_EXT_HV` reader - 0: Normal operation 1: Uses external turbo pulse"]
pub type TurboExtHvR = crate::BitReader;
#[doc = "Field `TURBO_EXT_HV` writer - 0: Normal operation 1: Uses external turbo pulse"]
pub type TurboExtHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPDAC_HWCTL_DIS_HV` reader - 0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
pub type NpdacHwctlDisHvR = crate::BitReader;
#[doc = "Field `NPDAC_HWCTL_DIS_HV` writer - 0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
pub type NpdacHwctlDisHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FM_READY_DIS_HV` reader - 0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
pub type FmReadyDisHvR = crate::BitReader;
#[doc = "Field `FM_READY_DIS_HV` writer - 0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
pub type FmReadyDisHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSX8_EN_ALL_HV` reader - 0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
pub type Ersx8EnAllHvR = crate::BitReader;
#[doc = "Field `ERSX8_EN_ALL_HV` writer - 0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
pub type Ersx8EnAllHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_LOAD_ONCE_HV` reader - 0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
pub type DisableLoadOnceHvR = crate::BitReader;
#[doc = "Field `DISABLE_LOAD_ONCE_HV` writer - 0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
pub type DisableLoadOnceHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE7_HV` reader - N/A"]
pub type Spare7HvR = crate::FieldReader;
#[doc = "Field `SPARE7_HV` writer - N/A"]
pub type Spare7HvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE7_ULP_HV` reader - N/A"]
pub type Spare7UlpHvR = crate::FieldReader;
#[doc = "Field `SPARE7_ULP_HV` writer - N/A"]
pub type Spare7UlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPARE7_LP_HV` reader - N/A"]
pub type Spare7LpHvR = crate::FieldReader;
#[doc = "Field `SPARE7_LP_HV` writer - N/A"]
pub type Spare7LpHvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
    #[inline(always)]
    pub fn ersx8_clk_sel_hv(&self) -> Ersx8ClkSelHvR {
        Ersx8ClkSelHvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 0: Normal operation 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn fm_active_hv(&self) -> FmActiveHvR {
        FmActiveHvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Normal operation 1: Uses external turbo pulse"]
    #[inline(always)]
    pub fn turbo_ext_hv(&self) -> TurboExtHvR {
        TurboExtHvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
    #[inline(always)]
    pub fn npdac_hwctl_dis_hv(&self) -> NpdacHwctlDisHvR {
        NpdacHwctlDisHvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
    #[inline(always)]
    pub fn fm_ready_dis_hv(&self) -> FmReadyDisHvR {
        FmReadyDisHvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
    #[inline(always)]
    pub fn ersx8_en_all_hv(&self) -> Ersx8EnAllHvR {
        Ersx8EnAllHvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
    #[inline(always)]
    pub fn disable_load_once_hv(&self) -> DisableLoadOnceHvR {
        DisableLoadOnceHvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn spare7_hv(&self) -> Spare7HvR {
        Spare7HvR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:14 - N/A"]
    #[inline(always)]
    pub fn spare7_ulp_hv(&self) -> Spare7UlpHvR {
        Spare7UlpHvR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - N/A"]
    #[inline(always)]
    pub fn spare7_lp_hv(&self) -> Spare7LpHvR {
        Spare7LpHvR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
    #[inline(always)]
    #[must_use]
    pub fn ersx8_clk_sel_hv(&mut self) -> Ersx8ClkSelHvW<CalCtl7Spec> {
        Ersx8ClkSelHvW::new(self, 0)
    }
    #[doc = "Bit 2 - 0: Normal operation 1: Forces FM SYS in active mode"]
    #[inline(always)]
    #[must_use]
    pub fn fm_active_hv(&mut self) -> FmActiveHvW<CalCtl7Spec> {
        FmActiveHvW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: Normal operation 1: Uses external turbo pulse"]
    #[inline(always)]
    #[must_use]
    pub fn turbo_ext_hv(&mut self) -> TurboExtHvW<CalCtl7Spec> {
        TurboExtHvW::new(self, 3)
    }
    #[doc = "Bit 4 - 0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
    #[inline(always)]
    #[must_use]
    pub fn npdac_hwctl_dis_hv(&mut self) -> NpdacHwctlDisHvW<CalCtl7Spec> {
        NpdacHwctlDisHvW::new(self, 4)
    }
    #[doc = "Bit 5 - 0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
    #[inline(always)]
    #[must_use]
    pub fn fm_ready_dis_hv(&mut self) -> FmReadyDisHvW<CalCtl7Spec> {
        FmReadyDisHvW::new(self, 5)
    }
    #[doc = "Bit 6 - 0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
    #[inline(always)]
    #[must_use]
    pub fn ersx8_en_all_hv(&mut self) -> Ersx8EnAllHvW<CalCtl7Spec> {
        Ersx8EnAllHvW::new(self, 6)
    }
    #[doc = "Bit 7 - 0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
    #[inline(always)]
    #[must_use]
    pub fn disable_load_once_hv(&mut self) -> DisableLoadOnceHvW<CalCtl7Spec> {
        DisableLoadOnceHvW::new(self, 7)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spare7_hv(&mut self) -> Spare7HvW<CalCtl7Spec> {
        Spare7HvW::new(self, 8)
    }
    #[doc = "Bits 10:14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spare7_ulp_hv(&mut self) -> Spare7UlpHvW<CalCtl7Spec> {
        Spare7UlpHvW::new(self, 10)
    }
    #[doc = "Bits 15:19 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spare7_lp_hv(&mut self) -> Spare7LpHvW<CalCtl7Spec> {
        Spare7LpHvW::new(self, 15)
    }
}
#[doc = "Cal control\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtl7Spec;
impl crate::RegisterSpec for CalCtl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl7::R`](R) reader structure"]
impl crate::Readable for CalCtl7Spec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl7::W`](W) writer structure"]
impl crate::Writable for CalCtl7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL7 to value 0"]
impl crate::Resettable for CalCtl7Spec {
    const RESET_VALUE: u32 = 0;
}
