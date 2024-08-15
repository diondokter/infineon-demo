#[doc = "Register `BOOT_CTRL_R` reader"]
pub type R = crate::R<BootCtrlRSpec>;
#[doc = "Register `BOOT_CTRL_R` writer"]
pub type W = crate::W<BootCtrlRSpec>;
#[doc = "Field `MAN_BOOT_EN` reader - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type ManBootEnR = crate::BitReader;
#[doc = "Field `MAN_BOOT_EN` writer - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type ManBootEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALIDATE_BOOT` writer - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: - 0x1 (TRUE): Validate Mandatory boot enable bit - 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
pub type ValidateBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ACK_ENABLE` reader - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
pub type BootAckEnableR = crate::BitReader;
#[doc = "Field `BOOT_ACK_ENABLE` writer - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
pub type BootAckEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_TOUT_CNT` reader - N/A"]
pub type BootToutCntR = crate::FieldReader;
#[doc = "Field `BOOT_TOUT_CNT` writer - N/A"]
pub type BootToutCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    pub fn man_boot_en(&self) -> ManBootEnR {
        ManBootEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(&self) -> BootAckEnableR {
        BootAckEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    pub fn boot_tout_cnt(&self) -> BootToutCntR {
        BootToutCntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    #[must_use]
    pub fn man_boot_en(&mut self) -> ManBootEnW<BootCtrlRSpec> {
        ManBootEnW::new(self, 0)
    }
    #[doc = "Bit 7 - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: - 0x1 (TRUE): Validate Mandatory boot enable bit - 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn validate_boot(&mut self) -> ValidateBootW<BootCtrlRSpec> {
        ValidateBootW::new(self, 7)
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_enable(&mut self) -> BootAckEnableW<BootCtrlRSpec> {
        BootAckEnableW::new(self, 8)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn boot_tout_cnt(&mut self) -> BootToutCntW<BootCtrlRSpec> {
        BootToutCntW::new(self, 12)
    }
}
#[doc = "eMMC Boot Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootCtrlRSpec;
impl crate::RegisterSpec for BootCtrlRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`boot_ctrl_r::R`](R) reader structure"]
impl crate::Readable for BootCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_ctrl_r::W`](W) writer structure"]
impl crate::Writable for BootCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BOOT_CTRL_R to value 0"]
impl crate::Resettable for BootCtrlRSpec {
    const RESET_VALUE: u16 = 0;
}
