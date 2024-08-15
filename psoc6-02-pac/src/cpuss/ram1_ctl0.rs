#[doc = "Register `RAM1_CTL0` reader"]
pub type R = crate::R<Ram1Ctl0Spec>;
#[doc = "Register `RAM1_CTL0` writer"]
pub type W = crate::W<Ram1Ctl0Spec>;
#[doc = "Field `SLOW_WS` reader - See RAM0_CTL."]
pub type SlowWsR = crate::FieldReader;
#[doc = "Field `SLOW_WS` writer - See RAM0_CTL."]
pub type SlowWsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAST_WS` reader - See RAM0_CTL."]
pub type FastWsR = crate::FieldReader;
#[doc = "Field `FAST_WS` writer - See RAM0_CTL."]
pub type FastWsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_EN` reader - See RAM0_CTL."]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - See RAM0_CTL."]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_AUTO_CORRECT` reader - See RAM0_CTL."]
pub type EccAutoCorrectR = crate::BitReader;
#[doc = "Field `ECC_AUTO_CORRECT` writer - See RAM0_CTL."]
pub type EccAutoCorrectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - See RAM0_CTL."]
pub type EccInjEnR = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - See RAM0_CTL."]
pub type EccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - See RAM0_CTL."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SlowWsR {
        SlowWsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - See RAM0_CTL."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FastWsR {
        FastWsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_auto_correct(&self) -> EccAutoCorrectR {
        EccAutoCorrectR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> EccInjEnR {
        EccInjEnR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SlowWsW<Ram1Ctl0Spec> {
        SlowWsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FastWsW<Ram1Ctl0Spec> {
        FastWsW::new(self, 8)
    }
    #[doc = "Bit 16 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<Ram1Ctl0Spec> {
        EccEnW::new(self, 16)
    }
    #[doc = "Bit 17 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_auto_correct(&mut self) -> EccAutoCorrectW<Ram1Ctl0Spec> {
        EccAutoCorrectW::new(self, 17)
    }
    #[doc = "Bit 18 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> EccInjEnW<Ram1Ctl0Spec> {
        EccInjEnW::new(self, 18)
    }
}
#[doc = "RAM 1 control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram1Ctl0Spec;
impl crate::RegisterSpec for Ram1Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1_ctl0::R`](R) reader structure"]
impl crate::Readable for Ram1Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ram1_ctl0::W`](W) writer structure"]
impl crate::Writable for Ram1Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM1_CTL0 to value 0x0003_0001"]
impl crate::Resettable for Ram1Ctl0Spec {
    const RESET_VALUE: u32 = 0x0003_0001;
}
