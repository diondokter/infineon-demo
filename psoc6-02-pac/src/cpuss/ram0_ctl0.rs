#[doc = "Register `RAM0_CTL0` reader"]
pub type R = crate::R<Ram0Ctl0Spec>;
#[doc = "Register `RAM0_CTL0` writer"]
pub type W = crate::W<Ram0Ctl0Spec>;
#[doc = "Field `SLOW_WS` reader - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type SlowWsR = crate::FieldReader;
#[doc = "Field `SLOW_WS` writer - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type SlowWsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAST_WS` reader - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FastWsR = crate::FieldReader;
#[doc = "Field `FAST_WS` writer - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FastWsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_AUTO_CORRECT` reader - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
pub type EccAutoCorrectR = crate::BitReader;
#[doc = "Field `ECC_AUTO_CORRECT` writer - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
pub type EccAutoCorrectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
pub type EccInjEnR = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
pub type EccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SlowWsR {
        SlowWsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FastWsR {
        FastWsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    pub fn ecc_auto_correct(&self) -> EccAutoCorrectR {
        EccAutoCorrectR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> EccInjEnR {
        EccInjEnR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SlowWsW<Ram0Ctl0Spec> {
        SlowWsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FastWsW<Ram0Ctl0Spec> {
        FastWsW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<Ram0Ctl0Spec> {
        EccEnW::new(self, 16)
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_auto_correct(&mut self) -> EccAutoCorrectW<Ram0Ctl0Spec> {
        EccAutoCorrectW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> EccInjEnW<Ram0Ctl0Spec> {
        EccInjEnW::new(self, 18)
    }
}
#[doc = "RAM 0 control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram0Ctl0Spec;
impl crate::RegisterSpec for Ram0Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0_ctl0::R`](R) reader structure"]
impl crate::Readable for Ram0Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ram0_ctl0::W`](W) writer structure"]
impl crate::Writable for Ram0Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM0_CTL0 to value 0x0003_0001"]
impl crate::Resettable for Ram0Ctl0Spec {
    const RESET_VALUE: u32 = 0x0003_0001;
}
