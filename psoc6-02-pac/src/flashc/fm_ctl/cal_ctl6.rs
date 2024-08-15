#[doc = "Register `CAL_CTL6` reader"]
pub type R = crate::R<CalCtl6Spec>;
#[doc = "Register `CAL_CTL6` writer"]
pub type W = crate::W<CalCtl6Spec>;
#[doc = "Field `SA_CTL_TRIM_T1_ULP_HV` reader - clk_trk delay"]
pub type SaCtlTrimT1UlpHvR = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T1_ULP_HV` writer - clk_trk delay"]
pub type SaCtlTrimT1UlpHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA_CTL_TRIM_T4_ULP_HV` reader - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SaCtlTrimT4UlpHvR = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T4_ULP_HV` writer - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SaCtlTrimT4UlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T5_ULP_HV` reader - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SaCtlTrimT5UlpHvR = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T5_ULP_HV` writer - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SaCtlTrimT5UlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T6_ULP_HV` reader - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
pub type SaCtlTrimT6UlpHvR = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T6_ULP_HV` writer - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
pub type SaCtlTrimT6UlpHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SA_CTL_TRIM_T8_ULP_HV` reader - saen3 pulse width trim (Current trim)"]
pub type SaCtlTrimT8UlpHvR = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T8_ULP_HV` writer - saen3 pulse width trim (Current trim)"]
pub type SaCtlTrimT8UlpHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA_CTL_TRIM_T1_LP_HV` reader - clk_trk delay"]
pub type SaCtlTrimT1LpHvR = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T1_LP_HV` writer - clk_trk delay"]
pub type SaCtlTrimT1LpHvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA_CTL_TRIM_T4_LP_HV` reader - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SaCtlTrimT4LpHvR = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T4_LP_HV` writer - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SaCtlTrimT4LpHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T5_LP_HV` reader - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SaCtlTrimT5LpHvR = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T5_LP_HV` writer - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SaCtlTrimT5LpHvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T6_LP_HV` reader - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
pub type SaCtlTrimT6LpHvR = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T6_LP_HV` writer - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
pub type SaCtlTrimT6LpHvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SA_CTL_TRIM_T8_LP_HV` reader - saen3 pulse width trim (Current trim)"]
pub type SaCtlTrimT8LpHvR = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T8_LP_HV` writer - saen3 pulse width trim (Current trim)"]
pub type SaCtlTrimT8LpHvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_ulp_hv(&self) -> SaCtlTrimT1UlpHvR {
        SaCtlTrimT1UlpHvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_ulp_hv(&self) -> SaCtlTrimT4UlpHvR {
        SaCtlTrimT4UlpHvR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_ulp_hv(&self) -> SaCtlTrimT5UlpHvR {
        SaCtlTrimT5UlpHvR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_ulp_hv(&self) -> SaCtlTrimT6UlpHvR {
        SaCtlTrimT6UlpHvR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_ulp_hv(&self) -> SaCtlTrimT8UlpHvR {
        SaCtlTrimT8UlpHvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_lp_hv(&self) -> SaCtlTrimT1LpHvR {
        SaCtlTrimT1LpHvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_lp_hv(&self) -> SaCtlTrimT4LpHvR {
        SaCtlTrimT4LpHvR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_lp_hv(&self) -> SaCtlTrimT5LpHvR {
        SaCtlTrimT5LpHvR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:18 - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_lp_hv(&self) -> SaCtlTrimT6LpHvR {
        SaCtlTrimT6LpHvR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_lp_hv(&self) -> SaCtlTrimT8LpHvR {
        SaCtlTrimT8LpHvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_trk delay"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t1_ulp_hv(&mut self) -> SaCtlTrimT1UlpHvW<CalCtl6Spec> {
        SaCtlTrimT1UlpHvW::new(self, 0)
    }
    #[doc = "Bits 1:3 - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t4_ulp_hv(&mut self) -> SaCtlTrimT4UlpHvW<CalCtl6Spec> {
        SaCtlTrimT4UlpHvW::new(self, 1)
    }
    #[doc = "Bits 4:6 - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t5_ulp_hv(&mut self) -> SaCtlTrimT5UlpHvW<CalCtl6Spec> {
        SaCtlTrimT5UlpHvW::new(self, 4)
    }
    #[doc = "Bits 7:8 - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t6_ulp_hv(&mut self) -> SaCtlTrimT6UlpHvW<CalCtl6Spec> {
        SaCtlTrimT6UlpHvW::new(self, 7)
    }
    #[doc = "Bit 9 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t8_ulp_hv(&mut self) -> SaCtlTrimT8UlpHvW<CalCtl6Spec> {
        SaCtlTrimT8UlpHvW::new(self, 9)
    }
    #[doc = "Bit 10 - clk_trk delay"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t1_lp_hv(&mut self) -> SaCtlTrimT1LpHvW<CalCtl6Spec> {
        SaCtlTrimT1LpHvW::new(self, 10)
    }
    #[doc = "Bits 11:13 - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t4_lp_hv(&mut self) -> SaCtlTrimT4LpHvW<CalCtl6Spec> {
        SaCtlTrimT4LpHvW::new(self, 11)
    }
    #[doc = "Bits 14:16 - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t5_lp_hv(&mut self) -> SaCtlTrimT5LpHvW<CalCtl6Spec> {
        SaCtlTrimT5LpHvW::new(self, 14)
    }
    #[doc = "Bits 17:18 - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t6_lp_hv(&mut self) -> SaCtlTrimT6LpHvW<CalCtl6Spec> {
        SaCtlTrimT6LpHvW::new(self, 17)
    }
    #[doc = "Bit 19 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t8_lp_hv(&mut self) -> SaCtlTrimT8LpHvW<CalCtl6Spec> {
        SaCtlTrimT8LpHvW::new(self, 19)
    }
}
#[doc = "SA trim LP/ULP\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalCtl6Spec;
impl crate::RegisterSpec for CalCtl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl6::R`](R) reader structure"]
impl crate::Readable for CalCtl6Spec {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl6::W`](W) writer structure"]
impl crate::Writable for CalCtl6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_CTL6 to value 0x0003_6f7f"]
impl crate::Resettable for CalCtl6Spec {
    const RESET_VALUE: u32 = 0x0003_6f7f;
}
