#[doc = "Register `CLK_CTRL_R` reader"]
pub type R = crate::R<ClkCtrlRSpec>;
#[doc = "Register `CLK_CTRL_R` writer"]
pub type W = crate::W<ClkCtrlRSpec>;
#[doc = "Field `INTERNAL_CLK_EN` reader - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. Certain registers are not accessible when this bit is off. So, to be safe turn it on for any register access. Values: - 0x0 (FALSE): Stop - 0x1 (TRUE): Oscillate"]
pub type InternalClkEnR = crate::BitReader;
#[doc = "Field `INTERNAL_CLK_EN` writer - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. Certain registers are not accessible when this bit is off. So, to be safe turn it on for any register access. Values: - 0x0 (FALSE): Stop - 0x1 (TRUE): Oscillate"]
pub type InternalClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERNAL_CLK_STABLE` reader - Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the Internal Clock Stable signal after the Internal Clock Enable bit is set to 1 and also reflects the synchronized value of the Card Clock Stable signal after the PLL Enable bit is set to 1. Values: - 0x0 (FALSE): Not Ready - 0x1 (TRUE): Ready"]
pub type InternalClkStableR = crate::BitReader;
#[doc = "Field `SD_CLK_EN` reader - SD/eMMC Clock Enable This bit stops the clk_card output when set to 0. The SDCLK Frequency Select bit can be changed when this bit is set to 0. Values: - 0x0 (FALSE): Disable providing clk_card - 0x1 (TRUE): Enable providing clk_card"]
pub type SdClkEnR = crate::BitReader;
#[doc = "Field `SD_CLK_EN` writer - SD/eMMC Clock Enable This bit stops the clk_card output when set to 0. The SDCLK Frequency Select bit can be changed when this bit is set to 0. Values: - 0x0 (FALSE): Disable providing clk_card - 0x1 (TRUE): Enable providing clk_card"]
pub type SdClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_ENABLE` reader - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). Values: - 0x0 (FALSE): PLL is in low power mode - 0x1 (TRUE): PLL is enabled"]
pub type PllEnableR = crate::BitReader;
#[doc = "Field `PLL_ENABLE` writer - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). Values: - 0x0 (FALSE): PLL is in low power mode - 0x1 (TRUE): PLL is enabled"]
pub type PllEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GEN_SELECT` reader - Clock Generator Select This bit is used to select the clock generator mode in SDCLK Frequency Select. Values: - 0x0 (FALSE): Divided Clock Mode - 0x1 (TRUE): Programmable Clock Mode"]
pub type ClkGenSelectR = crate::BitReader;
#[doc = "Field `CLK_GEN_SELECT` writer - Clock Generator Select This bit is used to select the clock generator mode in SDCLK Frequency Select. Values: - 0x0 (FALSE): Divided Clock Mode - 0x1 (TRUE): Programmable Clock Mode"]
pub type ClkGenSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPPER_FREQ_SEL` reader - These bits specify the upper 2 bits of 10-bit SDCLK Frequency Select control."]
pub type UpperFreqSelR = crate::FieldReader;
#[doc = "Field `UPPER_FREQ_SEL` writer - These bits specify the upper 2 bits of 10-bit SDCLK Frequency Select control."]
pub type UpperFreqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREQ_SEL` reader - SDCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. 10-bit Divided Clock Mode: - 0x3FF - 1/2046 Divided clock - .......... - N - 1/2N Divided Clock - .......... - 0x002 - 1/4 Divided Clock - 0x001 - 1/2 Divided Clock - 0x000 - Base clock (10MHz - 255 MHz)"]
pub type FreqSelR = crate::FieldReader;
#[doc = "Field `FREQ_SEL` writer - SDCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. 10-bit Divided Clock Mode: - 0x3FF - 1/2046 Divided clock - .......... - N - 1/2N Divided Clock - .......... - 0x002 - 1/4 Divided Clock - 0x001 - 1/2 Divided Clock - 0x000 - Base clock (10MHz - 255 MHz)"]
pub type FreqSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. Certain registers are not accessible when this bit is off. So, to be safe turn it on for any register access. Values: - 0x0 (FALSE): Stop - 0x1 (TRUE): Oscillate"]
    #[inline(always)]
    pub fn internal_clk_en(&self) -> InternalClkEnR {
        InternalClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the Internal Clock Stable signal after the Internal Clock Enable bit is set to 1 and also reflects the synchronized value of the Card Clock Stable signal after the PLL Enable bit is set to 1. Values: - 0x0 (FALSE): Not Ready - 0x1 (TRUE): Ready"]
    #[inline(always)]
    pub fn internal_clk_stable(&self) -> InternalClkStableR {
        InternalClkStableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD/eMMC Clock Enable This bit stops the clk_card output when set to 0. The SDCLK Frequency Select bit can be changed when this bit is set to 0. Values: - 0x0 (FALSE): Disable providing clk_card - 0x1 (TRUE): Enable providing clk_card"]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SdClkEnR {
        SdClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). Values: - 0x0 (FALSE): PLL is in low power mode - 0x1 (TRUE): PLL is enabled"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PllEnableR {
        PllEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select This bit is used to select the clock generator mode in SDCLK Frequency Select. Values: - 0x0 (FALSE): Divided Clock Mode - 0x1 (TRUE): Programmable Clock Mode"]
    #[inline(always)]
    pub fn clk_gen_select(&self) -> ClkGenSelectR {
        ClkGenSelectR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - These bits specify the upper 2 bits of 10-bit SDCLK Frequency Select control."]
    #[inline(always)]
    pub fn upper_freq_sel(&self) -> UpperFreqSelR {
        UpperFreqSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. 10-bit Divided Clock Mode: - 0x3FF - 1/2046 Divided clock - .......... - N - 1/2N Divided Clock - .......... - 0x002 - 1/4 Divided Clock - 0x001 - 1/2 Divided Clock - 0x000 - Base clock (10MHz - 255 MHz)"]
    #[inline(always)]
    pub fn freq_sel(&self) -> FreqSelR {
        FreqSelR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. Certain registers are not accessible when this bit is off. So, to be safe turn it on for any register access. Values: - 0x0 (FALSE): Stop - 0x1 (TRUE): Oscillate"]
    #[inline(always)]
    #[must_use]
    pub fn internal_clk_en(&mut self) -> InternalClkEnW<ClkCtrlRSpec> {
        InternalClkEnW::new(self, 0)
    }
    #[doc = "Bit 2 - SD/eMMC Clock Enable This bit stops the clk_card output when set to 0. The SDCLK Frequency Select bit can be changed when this bit is set to 0. Values: - 0x0 (FALSE): Disable providing clk_card - 0x1 (TRUE): Enable providing clk_card"]
    #[inline(always)]
    #[must_use]
    pub fn sd_clk_en(&mut self) -> SdClkEnW<ClkCtrlRSpec> {
        SdClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). Values: - 0x0 (FALSE): PLL is in low power mode - 0x1 (TRUE): PLL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pll_enable(&mut self) -> PllEnableW<ClkCtrlRSpec> {
        PllEnableW::new(self, 3)
    }
    #[doc = "Bit 5 - Clock Generator Select This bit is used to select the clock generator mode in SDCLK Frequency Select. Values: - 0x0 (FALSE): Divided Clock Mode - 0x1 (TRUE): Programmable Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gen_select(&mut self) -> ClkGenSelectW<ClkCtrlRSpec> {
        ClkGenSelectW::new(self, 5)
    }
    #[doc = "Bits 6:7 - These bits specify the upper 2 bits of 10-bit SDCLK Frequency Select control."]
    #[inline(always)]
    #[must_use]
    pub fn upper_freq_sel(&mut self) -> UpperFreqSelW<ClkCtrlRSpec> {
        UpperFreqSelW::new(self, 6)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. 10-bit Divided Clock Mode: - 0x3FF - 1/2046 Divided clock - .......... - N - 1/2N Divided Clock - .......... - 0x002 - 1/4 Divided Clock - 0x001 - 1/2 Divided Clock - 0x000 - Base clock (10MHz - 255 MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn freq_sel(&mut self) -> FreqSelW<ClkCtrlRSpec> {
        FreqSelW::new(self, 8)
    }
}
#[doc = "Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCtrlRSpec;
impl crate::RegisterSpec for ClkCtrlRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clk_ctrl_r::R`](R) reader structure"]
impl crate::Readable for ClkCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ctrl_r::W`](W) writer structure"]
impl crate::Writable for ClkCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CLK_CTRL_R to value 0"]
impl crate::Resettable for ClkCtrlRSpec {
    const RESET_VALUE: u16 = 0;
}
