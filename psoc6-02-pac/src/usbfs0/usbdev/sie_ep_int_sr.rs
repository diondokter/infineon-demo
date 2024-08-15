#[doc = "Register `SIE_EP_INT_SR` reader"]
pub type R = crate::R<SieEpIntSrSpec>;
#[doc = "Register `SIE_EP_INT_SR` writer"]
pub type W = crate::W<SieEpIntSrSpec>;
#[doc = "Field `EP1_INTR` reader - Interrupt status for EP1"]
pub type Ep1IntrR = crate::BitReader;
#[doc = "Field `EP1_INTR` writer - Interrupt status for EP1"]
pub type Ep1IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_INTR` reader - Interrupt status for EP2"]
pub type Ep2IntrR = crate::BitReader;
#[doc = "Field `EP2_INTR` writer - Interrupt status for EP2"]
pub type Ep2IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_INTR` reader - Interrupt status for EP3"]
pub type Ep3IntrR = crate::BitReader;
#[doc = "Field `EP3_INTR` writer - Interrupt status for EP3"]
pub type Ep3IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_INTR` reader - Interrupt status for EP4"]
pub type Ep4IntrR = crate::BitReader;
#[doc = "Field `EP4_INTR` writer - Interrupt status for EP4"]
pub type Ep4IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_INTR` reader - Interrupt status for EP5"]
pub type Ep5IntrR = crate::BitReader;
#[doc = "Field `EP5_INTR` writer - Interrupt status for EP5"]
pub type Ep5IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_INTR` reader - Interrupt status for EP6"]
pub type Ep6IntrR = crate::BitReader;
#[doc = "Field `EP6_INTR` writer - Interrupt status for EP6"]
pub type Ep6IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_INTR` reader - Interrupt status for EP7"]
pub type Ep7IntrR = crate::BitReader;
#[doc = "Field `EP7_INTR` writer - Interrupt status for EP7"]
pub type Ep7IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_INTR` reader - Interrupt status for EP8"]
pub type Ep8IntrR = crate::BitReader;
#[doc = "Field `EP8_INTR` writer - Interrupt status for EP8"]
pub type Ep8IntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> Ep1IntrR {
        Ep1IntrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> Ep2IntrR {
        Ep2IntrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> Ep3IntrR {
        Ep3IntrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> Ep4IntrR {
        Ep4IntrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> Ep5IntrR {
        Ep5IntrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> Ep6IntrR {
        Ep6IntrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> Ep7IntrR {
        Ep7IntrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> Ep8IntrR {
        Ep8IntrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_intr(&mut self) -> Ep1IntrW<SieEpIntSrSpec> {
        Ep1IntrW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_intr(&mut self) -> Ep2IntrW<SieEpIntSrSpec> {
        Ep2IntrW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_intr(&mut self) -> Ep3IntrW<SieEpIntSrSpec> {
        Ep3IntrW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_intr(&mut self) -> Ep4IntrW<SieEpIntSrSpec> {
        Ep4IntrW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_intr(&mut self) -> Ep5IntrW<SieEpIntSrSpec> {
        Ep5IntrW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_intr(&mut self) -> Ep6IntrW<SieEpIntSrSpec> {
        Ep6IntrW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_intr(&mut self) -> Ep7IntrW<SieEpIntSrSpec> {
        Ep7IntrW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_intr(&mut self) -> Ep8IntrW<SieEpIntSrSpec> {
        Ep8IntrW::new(self, 7)
    }
}
#[doc = "USB SIE Data Endpoint Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep_int_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep_int_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SieEpIntSrSpec;
impl crate::RegisterSpec for SieEpIntSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep_int_sr::R`](R) reader structure"]
impl crate::Readable for SieEpIntSrSpec {}
#[doc = "`write(|w| ..)` method takes [`sie_ep_int_sr::W`](W) writer structure"]
impl crate::Writable for SieEpIntSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIE_EP_INT_SR to value 0"]
impl crate::Resettable for SieEpIntSrSpec {
    const RESET_VALUE: u32 = 0;
}
