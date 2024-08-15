#[doc = "Register `DIV_CMD` reader"]
pub type R = crate::R<DivCmdSpec>;
#[doc = "Register `DIV_CMD` writer"]
pub type W = crate::W<DivCmdSpec>;
#[doc = "Field `DIV_SEL` reader - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
pub type DivSelR = crate::FieldReader;
#[doc = "Field `DIV_SEL` writer - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
pub type DivSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TYPE_SEL` reader - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type TypeSelR = crate::FieldReader;
#[doc = "Field `TYPE_SEL` writer - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type TypeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA_DIV_SEL` reader - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '255' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
pub type PaDivSelR = crate::FieldReader;
#[doc = "Field `PA_DIV_SEL` writer - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '255' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
pub type PaDivSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_TYPE_SEL` reader - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type PaTypeSelR = crate::FieldReader;
#[doc = "Field `PA_TYPE_SEL` writer - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type PaTypeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DISABLE` reader - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
pub type DisableR = crate::BitReader;
#[doc = "Field `DISABLE` writer - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    pub fn div_sel(&self) -> DivSelR {
        DivSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn type_sel(&self) -> TypeSelR {
        TypeSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '255' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    pub fn pa_div_sel(&self) -> PaDivSelR {
        PaDivSelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn pa_type_sel(&self) -> PaTypeSelR {
        PaTypeSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    #[must_use]
    pub fn div_sel(&mut self) -> DivSelW<DivCmdSpec> {
        DivSelW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    #[must_use]
    pub fn type_sel(&mut self) -> TypeSelW<DivCmdSpec> {
        TypeSelW::new(self, 8)
    }
    #[doc = "Bits 16:23 - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '255' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    #[must_use]
    pub fn pa_div_sel(&mut self) -> PaDivSelW<DivCmdSpec> {
        PaDivSelW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    #[must_use]
    pub fn pa_type_sel(&mut self) -> PaTypeSelW<DivCmdSpec> {
        PaTypeSelW::new(self, 24)
    }
    #[doc = "Bit 30 - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DisableW<DivCmdSpec> {
        DisableW::new(self, 30)
    }
    #[doc = "Bit 31 - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<DivCmdSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Divider command\n\nYou can [`read`](crate::Reg::read) this register and get [`div_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivCmdSpec;
impl crate::RegisterSpec for DivCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_cmd::R`](R) reader structure"]
impl crate::Readable for DivCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`div_cmd::W`](W) writer structure"]
impl crate::Writable for DivCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_CMD to value 0x03ff_03ff"]
impl crate::Resettable for DivCmdSpec {
    const RESET_VALUE: u32 = 0x03ff_03ff;
}
