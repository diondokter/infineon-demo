#[doc = "Register `GP_OUT_R` reader"]
pub type R = crate::R<GpOutRSpec>;
#[doc = "Register `GP_OUT_R` writer"]
pub type W = crate::W<GpOutRSpec>;
#[doc = "Field `CARD_DETECT_EN` reader - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
pub type CardDetectEnR = crate::BitReader;
#[doc = "Field `CARD_DETECT_EN` writer - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
pub type CardDetectEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_MECH_WRITE_PROT_EN` reader - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
pub type CardMechWriteProtEnR = crate::BitReader;
#[doc = "Field `CARD_MECH_WRITE_PROT_EN` writer - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
pub type CardMechWriteProtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LED_CTRL_OE` reader - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
pub type LedCtrlOeR = crate::BitReader;
#[doc = "Field `LED_CTRL_OE` writer - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
pub type LedCtrlOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_CLOCK_OE` reader - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
pub type CardClockOeR = crate::BitReader;
#[doc = "Field `CARD_CLOCK_OE` writer - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
pub type CardClockOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_IF_PWR_EN_OE` reader - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
pub type CardIfPwrEnOeR = crate::BitReader;
#[doc = "Field `CARD_IF_PWR_EN_OE` writer - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
pub type CardIfPwrEnOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLT_SEL_OE` reader - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
pub type IoVoltSelOeR = crate::BitReader;
#[doc = "Field `IO_VOLT_SEL_OE` writer - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
pub type IoVoltSelOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_CLOCK_OUT_DLY` reader - N/A"]
pub type CardClockOutDlyR = crate::FieldReader;
#[doc = "Field `CARD_CLOCK_OUT_DLY` writer - N/A"]
pub type CardClockOutDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CARD_CLOCK_IN_DLY` reader - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
pub type CardClockInDlyR = crate::FieldReader;
#[doc = "Field `CARD_CLOCK_IN_DLY` writer - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
pub type CardClockInDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
    #[inline(always)]
    pub fn card_detect_en(&self) -> CardDetectEnR {
        CardDetectEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
    #[inline(always)]
    pub fn card_mech_write_prot_en(&self) -> CardMechWriteProtEnR {
        CardMechWriteProtEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
    #[inline(always)]
    pub fn led_ctrl_oe(&self) -> LedCtrlOeR {
        LedCtrlOeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
    #[inline(always)]
    pub fn card_clock_oe(&self) -> CardClockOeR {
        CardClockOeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
    #[inline(always)]
    pub fn card_if_pwr_en_oe(&self) -> CardIfPwrEnOeR {
        CardIfPwrEnOeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
    #[inline(always)]
    pub fn io_volt_sel_oe(&self) -> IoVoltSelOeR {
        IoVoltSelOeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn card_clock_out_dly(&self) -> CardClockOutDlyR {
        CardClockOutDlyR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
    #[inline(always)]
    pub fn card_clock_in_dly(&self) -> CardClockInDlyR {
        CardClockInDlyR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
    #[inline(always)]
    #[must_use]
    pub fn card_detect_en(&mut self) -> CardDetectEnW<GpOutRSpec> {
        CardDetectEnW::new(self, 0)
    }
    #[doc = "Bit 1 - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
    #[inline(always)]
    #[must_use]
    pub fn card_mech_write_prot_en(&mut self) -> CardMechWriteProtEnW<GpOutRSpec> {
        CardMechWriteProtEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
    #[inline(always)]
    #[must_use]
    pub fn led_ctrl_oe(&mut self) -> LedCtrlOeW<GpOutRSpec> {
        LedCtrlOeW::new(self, 2)
    }
    #[doc = "Bit 3 - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
    #[inline(always)]
    #[must_use]
    pub fn card_clock_oe(&mut self) -> CardClockOeW<GpOutRSpec> {
        CardClockOeW::new(self, 3)
    }
    #[doc = "Bit 4 - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
    #[inline(always)]
    #[must_use]
    pub fn card_if_pwr_en_oe(&mut self) -> CardIfPwrEnOeW<GpOutRSpec> {
        CardIfPwrEnOeW::new(self, 4)
    }
    #[doc = "Bit 5 - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
    #[inline(always)]
    #[must_use]
    pub fn io_volt_sel_oe(&mut self) -> IoVoltSelOeW<GpOutRSpec> {
        IoVoltSelOeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn card_clock_out_dly(&mut self) -> CardClockOutDlyW<GpOutRSpec> {
        CardClockOutDlyW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
    #[inline(always)]
    #[must_use]
    pub fn card_clock_in_dly(&mut self) -> CardClockInDlyW<GpOutRSpec> {
        CardClockInDlyW::new(self, 8)
    }
}
#[doc = "General Purpose Output register\n\nYou can [`read`](crate::Reg::read) this register and get [`gp_out_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_out_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpOutRSpec;
impl crate::RegisterSpec for GpOutRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_out_r::R`](R) reader structure"]
impl crate::Readable for GpOutRSpec {}
#[doc = "`write(|w| ..)` method takes [`gp_out_r::W`](W) writer structure"]
impl crate::Writable for GpOutRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GP_OUT_R to value 0"]
impl crate::Resettable for GpOutRSpec {
    const RESET_VALUE: u32 = 0;
}
