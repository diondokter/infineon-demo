#[doc = "Register `INTR_USBHOST_MASK` reader"]
pub type R = crate::R<IntrUsbhostMaskSpec>;
#[doc = "Register `INTR_USBHOST_MASK` writer"]
pub type W = crate::W<IntrUsbhostMaskSpec>;
#[doc = "Field `SOFIRQM` reader - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub type SofirqmR = crate::BitReader;
#[doc = "Field `SOFIRQM` writer - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub type SofirqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRQM` reader - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub type DirqmR = crate::BitReader;
#[doc = "Field `DIRQM` writer - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub type DirqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNIRQM` reader - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub type CnnirqmR = crate::BitReader;
#[doc = "Field `CNNIRQM` writer - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub type CnnirqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIRQM` reader - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub type CmpirqmR = crate::BitReader;
#[doc = "Field `CMPIRQM` writer - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub type CmpirqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URIRQM` reader - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub type UrirqmR = crate::BitReader;
#[doc = "Field `URIRQM` writer - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub type UrirqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKIRQM` reader - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub type RwkirqmR = crate::BitReader;
#[doc = "Field `RWKIRQM` writer - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub type RwkirqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - N/A"]
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCANM` reader - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub type TcanmR = crate::BitReader;
#[doc = "Field `TCANM` writer - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub type TcanmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&self) -> SofirqmR {
        SofirqmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&self) -> DirqmR {
        DirqmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&self) -> CnnirqmR {
        CnnirqmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&self) -> CmpirqmR {
        CmpirqmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&self) -> UrirqmR {
        UrirqmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&self) -> RwkirqmR {
        RwkirqmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&self) -> TcanmR {
        TcanmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn sofirqm(&mut self) -> SofirqmW<IntrUsbhostMaskSpec> {
        SofirqmW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn dirqm(&mut self) -> DirqmW<IntrUsbhostMaskSpec> {
        DirqmW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn cnnirqm(&mut self) -> CnnirqmW<IntrUsbhostMaskSpec> {
        CnnirqmW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn cmpirqm(&mut self) -> CmpirqmW<IntrUsbhostMaskSpec> {
        CmpirqmW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn urirqm(&mut self) -> UrirqmW<IntrUsbhostMaskSpec> {
        UrirqmW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn rwkirqm(&mut self) -> RwkirqmW<IntrUsbhostMaskSpec> {
        RwkirqmW::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_6(&mut self) -> Rsvd6W<IntrUsbhostMaskSpec> {
        Rsvd6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn tcanm(&mut self) -> TcanmW<IntrUsbhostMaskSpec> {
        TcanmW::new(self, 7)
    }
}
#[doc = "Interrupt USB Host Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_usbhost_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrUsbhostMaskSpec;
impl crate::RegisterSpec for IntrUsbhostMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost_mask::R`](R) reader structure"]
impl crate::Readable for IntrUsbhostMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_usbhost_mask::W`](W) writer structure"]
impl crate::Writable for IntrUsbhostMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_USBHOST_MASK to value 0"]
impl crate::Resettable for IntrUsbhostMaskSpec {
    const RESET_VALUE: u32 = 0;
}
