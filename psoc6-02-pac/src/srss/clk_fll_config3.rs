#[doc = "Register `CLK_FLL_CONFIG3` reader"]
pub type R = crate::R<ClkFllConfig3Spec>;
#[doc = "Register `CLK_FLL_CONFIG3` writer"]
pub type W = crate::W<ClkFllConfig3Spec>;
#[doc = "Field `FLL_LF_IGAIN` reader - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FllLfIgainR = crate::FieldReader;
#[doc = "Field `FLL_LF_IGAIN` writer - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FllLfIgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLL_LF_PGAIN` reader - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FllLfPgainR = crate::FieldReader;
#[doc = "Field `FLL_LF_PGAIN` writer - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FllLfPgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SETTLING_COUNT` reader - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
pub type SettlingCountR = crate::FieldReader<u16>;
#[doc = "Field `SETTLING_COUNT` writer - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
pub type SettlingCountW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BypassSel {
    #[doc = "0: N/A"]
    Auto = 0,
    #[doc = "1: N/A"]
    Auto1 = 1,
    #[doc = "2: Select FLL reference input (bypass mode). Ignores lock indicator"]
    FllRef = 2,
    #[doc = "3: Select FLL output. Ignores lock indicator."]
    FllOut = 3,
}
impl From<BypassSel> for u8 {
    #[inline(always)]
    fn from(variant: BypassSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BypassSel {
    type Ux = u8;
}
impl crate::IsEnum for BypassSel {}
#[doc = "Field `BYPASS_SEL` reader - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
pub type BypassSelR = crate::FieldReader<BypassSel>;
impl BypassSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassSel {
        match self.bits {
            0 => BypassSel::Auto,
            1 => BypassSel::Auto1,
            2 => BypassSel::FllRef,
            3 => BypassSel::FllOut,
            _ => unreachable!(),
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BypassSel::Auto
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_auto1(&self) -> bool {
        *self == BypassSel::Auto1
    }
    #[doc = "Select FLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn is_fll_ref(&self) -> bool {
        *self == BypassSel::FllRef
    }
    #[doc = "Select FLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn is_fll_out(&self) -> bool {
        *self == BypassSel::FllOut
    }
}
#[doc = "Field `BYPASS_SEL` writer - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
pub type BypassSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, BypassSel, crate::Safe>;
impl<'a, REG> BypassSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::Auto)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn auto1(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::Auto1)
    }
    #[doc = "Select FLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn fll_ref(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::FllRef)
    }
    #[doc = "Select FLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn fll_out(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::FllOut)
    }
}
impl R {
    #[doc = "Bits 0:3 - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_igain(&self) -> FllLfIgainR {
        FllLfIgainR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_pgain(&self) -> FllLfPgainR {
        FllLfPgainR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub fn settling_count(&self) -> SettlingCountR {
        SettlingCountR::new(((self.bits >> 8) & 0x1fff) as u16)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BypassSelR {
        BypassSelR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    #[must_use]
    pub fn fll_lf_igain(&mut self) -> FllLfIgainW<ClkFllConfig3Spec> {
        FllLfIgainW::new(self, 0)
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    #[must_use]
    pub fn fll_lf_pgain(&mut self) -> FllLfPgainW<ClkFllConfig3Spec> {
        FllLfPgainW::new(self, 4)
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn settling_count(&mut self) -> SettlingCountW<ClkFllConfig3Spec> {
        SettlingCountW::new(self, 8)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_sel(&mut self) -> BypassSelW<ClkFllConfig3Spec> {
        BypassSelW::new(self, 28)
    }
}
#[doc = "FLL Configuration Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_config3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_config3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkFllConfig3Spec;
impl crate::RegisterSpec for ClkFllConfig3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_config3::R`](R) reader structure"]
impl crate::Readable for ClkFllConfig3Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_config3::W`](W) writer structure"]
impl crate::Writable for ClkFllConfig3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG3 to value 0x2800"]
impl crate::Resettable for ClkFllConfig3Spec {
    const RESET_VALUE: u32 = 0x2800;
}
