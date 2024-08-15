#[doc = "Register `PWR_CTRL_R` reader"]
pub type R = crate::R<PwrCtrlRSpec>;
#[doc = "Register `PWR_CTRL_R` writer"]
pub type W = crate::W<PwrCtrlRSpec>;
#[doc = "Field `SD_BUS_PWR_VDD1` reader - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
pub type SdBusPwrVdd1R = crate::BitReader;
#[doc = "Field `SD_BUS_PWR_VDD1` writer - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
pub type SdBusPwrVdd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_BUS_VOL_VDD1` reader - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
pub type SdBusVolVdd1R = crate::FieldReader;
#[doc = "Field `SD_BUS_VOL_VDD1` writer - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
pub type SdBusVolVdd1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
    #[inline(always)]
    pub fn sd_bus_pwr_vdd1(&self) -> SdBusPwrVdd1R {
        SdBusPwrVdd1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
    #[inline(always)]
    pub fn sd_bus_vol_vdd1(&self) -> SdBusVolVdd1R {
        SdBusVolVdd1R::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_pwr_vdd1(&mut self) -> SdBusPwrVdd1W<PwrCtrlRSpec> {
        SdBusPwrVdd1W::new(self, 0)
    }
    #[doc = "Bits 1:3 - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_vol_vdd1(&mut self) -> SdBusVolVdd1W<PwrCtrlRSpec> {
        SdBusVolVdd1W::new(self, 1)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCtrlRSpec;
impl crate::RegisterSpec for PwrCtrlRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwr_ctrl_r::R`](R) reader structure"]
impl crate::Readable for PwrCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_ctrl_r::W`](W) writer structure"]
impl crate::Writable for PwrCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWR_CTRL_R to value 0"]
impl crate::Resettable for PwrCtrlRSpec {
    const RESET_VALUE: u8 = 0;
}
