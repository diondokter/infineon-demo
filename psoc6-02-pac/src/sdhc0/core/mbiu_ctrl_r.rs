#[doc = "Register `MBIU_CTRL_R` reader"]
pub type R = crate::R<MbiuCtrlRSpec>;
#[doc = "Register `MBIU_CTRL_R` writer"]
pub type W = crate::W<MbiuCtrlRSpec>;
#[doc = "Field `UNDEFL_INCR_EN` reader - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
pub type UndeflIncrEnR = crate::BitReader;
#[doc = "Field `UNDEFL_INCR_EN` writer - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
pub type UndeflIncrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_INCR4_EN` reader - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
pub type BurstIncr4EnR = crate::BitReader;
#[doc = "Field `BURST_INCR4_EN` writer - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
pub type BurstIncr4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_INCR8_EN` reader - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
pub type BurstIncr8EnR = crate::BitReader;
#[doc = "Field `BURST_INCR8_EN` writer - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
pub type BurstIncr8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_INCR16_EN` reader - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
pub type BurstIncr16EnR = crate::BitReader;
#[doc = "Field `BURST_INCR16_EN` writer - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
pub type BurstIncr16EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
    #[inline(always)]
    pub fn undefl_incr_en(&self) -> UndeflIncrEnR {
        UndeflIncrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr4_en(&self) -> BurstIncr4EnR {
        BurstIncr4EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr8_en(&self) -> BurstIncr8EnR {
        BurstIncr8EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr16_en(&self) -> BurstIncr16EnR {
        BurstIncr16EnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn undefl_incr_en(&mut self) -> UndeflIncrEnW<MbiuCtrlRSpec> {
        UndeflIncrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn burst_incr4_en(&mut self) -> BurstIncr4EnW<MbiuCtrlRSpec> {
        BurstIncr4EnW::new(self, 1)
    }
    #[doc = "Bit 2 - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn burst_incr8_en(&mut self) -> BurstIncr8EnW<MbiuCtrlRSpec> {
        BurstIncr8EnW::new(self, 2)
    }
    #[doc = "Bit 3 - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn burst_incr16_en(&mut self) -> BurstIncr16EnW<MbiuCtrlRSpec> {
        BurstIncr16EnW::new(self, 3)
    }
}
#[doc = "MBIU Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mbiu_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbiu_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbiuCtrlRSpec;
impl crate::RegisterSpec for MbiuCtrlRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mbiu_ctrl_r::R`](R) reader structure"]
impl crate::Readable for MbiuCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`mbiu_ctrl_r::W`](W) writer structure"]
impl crate::Writable for MbiuCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MBIU_CTRL_R to value 0x01"]
impl crate::Resettable for MbiuCtrlRSpec {
    const RESET_VALUE: u8 = 0x01;
}
