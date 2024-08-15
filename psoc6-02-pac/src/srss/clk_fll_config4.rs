#[doc = "Register `CLK_FLL_CONFIG4` reader"]
pub type R = crate::R<ClkFllConfig4Spec>;
#[doc = "Register `CLK_FLL_CONFIG4` writer"]
pub type W = crate::W<ClkFllConfig4Spec>;
#[doc = "Field `CCO_LIMIT` reader - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
pub type CcoLimitR = crate::FieldReader;
#[doc = "Field `CCO_LIMIT` writer - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
pub type CcoLimitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Frequency range of CCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CcoRange {
    #[doc = "0: Target frequency is in range \\[48, 64) MHz"]
    Range0 = 0,
    #[doc = "1: Target frequency is in range \\[64, 85) MHz"]
    Range1 = 1,
    #[doc = "2: Target frequency is in range \\[85, 113) MHz"]
    Range2 = 2,
    #[doc = "3: Target frequency is in range \\[113, 150) MHz"]
    Range3 = 3,
    #[doc = "4: Target frequency is in range \\[150, 200\\]
MHz"]
    Range4 = 4,
}
impl From<CcoRange> for u8 {
    #[inline(always)]
    fn from(variant: CcoRange) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CcoRange {
    type Ux = u8;
}
impl crate::IsEnum for CcoRange {}
#[doc = "Field `CCO_RANGE` reader - Frequency range of CCO"]
pub type CcoRangeR = crate::FieldReader<CcoRange>;
impl CcoRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CcoRange> {
        match self.bits {
            0 => Some(CcoRange::Range0),
            1 => Some(CcoRange::Range1),
            2 => Some(CcoRange::Range2),
            3 => Some(CcoRange::Range3),
            4 => Some(CcoRange::Range4),
            _ => None,
        }
    }
    #[doc = "Target frequency is in range \\[48, 64) MHz"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == CcoRange::Range0
    }
    #[doc = "Target frequency is in range \\[64, 85) MHz"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == CcoRange::Range1
    }
    #[doc = "Target frequency is in range \\[85, 113) MHz"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == CcoRange::Range2
    }
    #[doc = "Target frequency is in range \\[113, 150) MHz"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == CcoRange::Range3
    }
    #[doc = "Target frequency is in range \\[150, 200\\]
MHz"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == CcoRange::Range4
    }
}
#[doc = "Field `CCO_RANGE` writer - Frequency range of CCO"]
pub type CcoRangeW<'a, REG> = crate::FieldWriter<'a, REG, 3, CcoRange>;
impl<'a, REG> CcoRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target frequency is in range \\[48, 64) MHz"]
    #[inline(always)]
    pub fn range0(self) -> &'a mut crate::W<REG> {
        self.variant(CcoRange::Range0)
    }
    #[doc = "Target frequency is in range \\[64, 85) MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(CcoRange::Range1)
    }
    #[doc = "Target frequency is in range \\[85, 113) MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(CcoRange::Range2)
    }
    #[doc = "Target frequency is in range \\[113, 150) MHz"]
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(CcoRange::Range3)
    }
    #[doc = "Target frequency is in range \\[150, 200\\]
MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(CcoRange::Range4)
    }
}
#[doc = "Field `CCO_FREQ` reader - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
pub type CcoFreqR = crate::FieldReader<u16>;
#[doc = "Field `CCO_FREQ` writer - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
pub type CcoFreqW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CCO_HW_UPDATE_DIS` reader - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
pub type CcoHwUpdateDisR = crate::BitReader;
#[doc = "Field `CCO_HW_UPDATE_DIS` writer - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
pub type CcoHwUpdateDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCO_ENABLE` reader - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
pub type CcoEnableR = crate::BitReader;
#[doc = "Field `CCO_ENABLE` writer - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
pub type CcoEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn cco_limit(&self) -> CcoLimitR {
        CcoLimitR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    pub fn cco_range(&self) -> CcoRangeR {
        CcoRangeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub fn cco_freq(&self) -> CcoFreqR {
        CcoFreqR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub fn cco_hw_update_dis(&self) -> CcoHwUpdateDisR {
        CcoHwUpdateDisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn cco_enable(&self) -> CcoEnableR {
        CcoEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    #[must_use]
    pub fn cco_limit(&mut self) -> CcoLimitW<ClkFllConfig4Spec> {
        CcoLimitW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    #[must_use]
    pub fn cco_range(&mut self) -> CcoRangeW<ClkFllConfig4Spec> {
        CcoRangeW::new(self, 8)
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    #[must_use]
    pub fn cco_freq(&mut self) -> CcoFreqW<ClkFllConfig4Spec> {
        CcoFreqW::new(self, 16)
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    #[must_use]
    pub fn cco_hw_update_dis(&mut self) -> CcoHwUpdateDisW<ClkFllConfig4Spec> {
        CcoHwUpdateDisW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    #[must_use]
    pub fn cco_enable(&mut self) -> CcoEnableW<ClkFllConfig4Spec> {
        CcoEnableW::new(self, 31)
    }
}
#[doc = "FLL Configuration Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_config4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_config4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkFllConfig4Spec;
impl crate::RegisterSpec for ClkFllConfig4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_config4::R`](R) reader structure"]
impl crate::Readable for ClkFllConfig4Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_config4::W`](W) writer structure"]
impl crate::Writable for ClkFllConfig4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG4 to value 0xff"]
impl crate::Resettable for ClkFllConfig4Spec {
    const RESET_VALUE: u32 = 0xff;
}
