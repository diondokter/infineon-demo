#[doc = "Register `POWER_CTL` reader"]
pub type R = crate::R<PowerCtlSpec>;
#[doc = "Register `POWER_CTL` writer"]
pub type W = crate::W<PowerCtlSpec>;
#[doc = "Field `SUSPEND` reader - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
pub type SuspendR = crate::BitReader;
#[doc = "Field `SUSPEND` writer - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_UP_EN` reader - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
pub type DpUpEnR = crate::BitReader;
#[doc = "Field `DP_UP_EN` writer - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
pub type DpUpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_BIG` reader - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
pub type DpBigR = crate::BitReader;
#[doc = "Field `DP_BIG` writer - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
pub type DpBigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_DOWN_EN` reader - Enables the ~15k pull down on the DP."]
pub type DpDownEnR = crate::BitReader;
#[doc = "Field `DP_DOWN_EN` writer - Enables the ~15k pull down on the DP."]
pub type DpDownEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_UP_EN` reader - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
pub type DmUpEnR = crate::BitReader;
#[doc = "Field `DM_UP_EN` writer - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
pub type DmUpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_BIG` reader - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
pub type DmBigR = crate::BitReader;
#[doc = "Field `DM_BIG` writer - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
pub type DmBigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_DOWN_EN` reader - Enables the ~15k pull down on the DP."]
pub type DmDownEnR = crate::BitReader;
#[doc = "Field `DM_DOWN_EN` writer - Enables the ~15k pull down on the DP."]
pub type DmDownEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DPO` reader - Enables the single ended receiver on D+."]
pub type EnableDpoR = crate::BitReader;
#[doc = "Field `ENABLE_DPO` writer - Enables the single ended receiver on D+."]
pub type EnableDpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DMO` reader - Enables the signle ended receiver on D-."]
pub type EnableDmoR = crate::BitReader;
#[doc = "Field `ENABLE_DMO` writer - Enables the signle ended receiver on D-."]
pub type EnableDmoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dp_up_en(&self) -> DpUpEnR {
        DpUpEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    pub fn dp_big(&self) -> DpBigR {
        DpBigR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dp_down_en(&self) -> DpDownEnR {
        DpDownEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dm_up_en(&self) -> DmUpEnR {
        DmUpEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    pub fn dm_big(&self) -> DmBigR {
        DmBigR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dm_down_en(&self) -> DmDownEnR {
        DmDownEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    pub fn enable_dpo(&self) -> EnableDpoR {
        EnableDpoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub fn enable_dmo(&self) -> EnableDmoR {
        EnableDmoR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SuspendW<PowerCtlSpec> {
        SuspendW::new(self, 2)
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dp_up_en(&mut self) -> DpUpEnW<PowerCtlSpec> {
        DpUpEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    #[must_use]
    pub fn dp_big(&mut self) -> DpBigW<PowerCtlSpec> {
        DpBigW::new(self, 17)
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    #[must_use]
    pub fn dp_down_en(&mut self) -> DpDownEnW<PowerCtlSpec> {
        DpDownEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dm_up_en(&mut self) -> DmUpEnW<PowerCtlSpec> {
        DmUpEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    #[must_use]
    pub fn dm_big(&mut self) -> DmBigW<PowerCtlSpec> {
        DmBigW::new(self, 20)
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    #[must_use]
    pub fn dm_down_en(&mut self) -> DmDownEnW<PowerCtlSpec> {
        DmDownEnW::new(self, 21)
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    #[must_use]
    pub fn enable_dpo(&mut self) -> EnableDpoW<PowerCtlSpec> {
        EnableDpoW::new(self, 28)
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    #[must_use]
    pub fn enable_dmo(&mut self) -> EnableDmoW<PowerCtlSpec> {
        EnableDmoW::new(self, 29)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`power_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerCtlSpec;
impl crate::RegisterSpec for PowerCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_ctl::R`](R) reader structure"]
impl crate::Readable for PowerCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_ctl::W`](W) writer structure"]
impl crate::Writable for PowerCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_CTL to value 0"]
impl crate::Resettable for PowerCtlSpec {
    const RESET_VALUE: u32 = 0;
}
