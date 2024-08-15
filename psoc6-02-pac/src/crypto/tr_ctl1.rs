#[doc = "Register `TR_CTL1` reader"]
pub type R = crate::R<TrCtl1Spec>;
#[doc = "Register `TR_CTL1` writer"]
pub type W = crate::W<TrCtl1Spec>;
#[doc = "Field `RO11_EN` reader - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
pub type Ro11EnR = crate::BitReader;
#[doc = "Field `RO11_EN` writer - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
pub type Ro11EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RO15_EN` reader - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
pub type Ro15EnR = crate::BitReader;
#[doc = "Field `RO15_EN` writer - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
pub type Ro15EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GARO15_EN` reader - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
pub type Garo15EnR = crate::BitReader;
#[doc = "Field `GARO15_EN` writer - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
pub type Garo15EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GARO31_EN` reader - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
pub type Garo31EnR = crate::BitReader;
#[doc = "Field `GARO31_EN` writer - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
pub type Garo31EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIRO15_EN` reader - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
pub type Firo15EnR = crate::BitReader;
#[doc = "Field `FIRO15_EN` writer - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
pub type Firo15EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIRO31_EN` reader - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
pub type Firo31EnR = crate::BitReader;
#[doc = "Field `FIRO31_EN` writer - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
pub type Firo31EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
    #[inline(always)]
    pub fn ro11_en(&self) -> Ro11EnR {
        Ro11EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
    #[inline(always)]
    pub fn ro15_en(&self) -> Ro15EnR {
        Ro15EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
    #[inline(always)]
    pub fn garo15_en(&self) -> Garo15EnR {
        Garo15EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    pub fn garo31_en(&self) -> Garo31EnR {
        Garo31EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
    #[inline(always)]
    pub fn firo15_en(&self) -> Firo15EnR {
        Firo15EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    pub fn firo31_en(&self) -> Firo31EnR {
        Firo31EnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn ro11_en(&mut self) -> Ro11EnW<TrCtl1Spec> {
        Ro11EnW::new(self, 0)
    }
    #[doc = "Bit 1 - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn ro15_en(&mut self) -> Ro15EnW<TrCtl1Spec> {
        Ro15EnW::new(self, 1)
    }
    #[doc = "Bit 2 - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn garo15_en(&mut self) -> Garo15EnW<TrCtl1Spec> {
        Garo15EnW::new(self, 2)
    }
    #[doc = "Bit 3 - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    #[must_use]
    pub fn garo31_en(&mut self) -> Garo31EnW<TrCtl1Spec> {
        Garo31EnW::new(self, 3)
    }
    #[doc = "Bit 4 - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn firo15_en(&mut self) -> Firo15EnW<TrCtl1Spec> {
        Firo15EnW::new(self, 4)
    }
    #[doc = "Bit 5 - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    #[must_use]
    pub fn firo31_en(&mut self) -> Firo31EnW<TrCtl1Spec> {
        Firo31EnW::new(self, 5)
    }
}
#[doc = "True random control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtl1Spec;
impl crate::RegisterSpec for TrCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctl1::R`](R) reader structure"]
impl crate::Readable for TrCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctl1::W`](W) writer structure"]
impl crate::Writable for TrCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTL1 to value 0"]
impl crate::Resettable for TrCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
