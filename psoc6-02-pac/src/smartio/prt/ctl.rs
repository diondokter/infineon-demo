#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `BYPASS` reader - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
pub type BypassR = crate::FieldReader;
#[doc = "Field `BYPASS` writer - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
pub type BypassW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLOCK_SRC` reader - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
pub type ClockSrcR = crate::FieldReader;
#[doc = "Field `CLOCK_SRC` writer - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
pub type ClockSrcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HLD_OVR` reader - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
pub type HldOvrR = crate::BitReader;
#[doc = "Field `HLD_OVR` writer - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
pub type HldOvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPELINE_EN` reader - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
pub type PipelineEnR = crate::BitReader;
#[doc = "Field `PIPELINE_EN` writer - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
pub type PipelineEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
    #[inline(always)]
    pub fn clock_src(&self) -> ClockSrcR {
        ClockSrcR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
    #[inline(always)]
    pub fn hld_ovr(&self) -> HldOvrR {
        HldOvrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    pub fn pipeline_en(&self) -> PipelineEnR {
        PipelineEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<CtlSpec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
    #[inline(always)]
    #[must_use]
    pub fn clock_src(&mut self) -> ClockSrcW<CtlSpec> {
        ClockSrcW::new(self, 8)
    }
    #[doc = "Bit 24 - IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
    #[inline(always)]
    #[must_use]
    pub fn hld_ovr(&mut self) -> HldOvrW<CtlSpec> {
        HldOvrW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pipeline_en(&mut self) -> PipelineEnW<CtlSpec> {
        PipelineEnW::new(self, 25)
    }
    #[doc = "Bit 31 - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0200_1400"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0200_1400;
}
