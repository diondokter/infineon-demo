#[doc = "Register `INTR_USBHOST_MASKED` reader"]
pub type R = crate::R<IntrUsbhostMaskedSpec>;
#[doc = "Field `SOFIRQED` reader - This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
pub type SofirqedR = crate::BitReader;
#[doc = "Field `DIRQED` reader - This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
pub type DirqedR = crate::BitReader;
#[doc = "Field `CNNIRQED` reader - This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
pub type CnnirqedR = crate::BitReader;
#[doc = "Field `CMPIRQED` reader - This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
pub type CmpirqedR = crate::BitReader;
#[doc = "Field `URIRQED` reader - This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
pub type UrirqedR = crate::BitReader;
#[doc = "Field `RWKIRQED` reader - This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
pub type RwkirqedR = crate::BitReader;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `TCANED` reader - This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
pub type TcanedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
    #[inline(always)]
    pub fn sofirqed(&self) -> SofirqedR {
        SofirqedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
    #[inline(always)]
    pub fn dirqed(&self) -> DirqedR {
        DirqedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
    #[inline(always)]
    pub fn cnnirqed(&self) -> CnnirqedR {
        CnnirqedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
    #[inline(always)]
    pub fn cmpirqed(&self) -> CmpirqedR {
        CmpirqedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
    #[inline(always)]
    pub fn urirqed(&self) -> UrirqedR {
        UrirqedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
    #[inline(always)]
    pub fn rwkirqed(&self) -> RwkirqedR {
        RwkirqedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
    #[inline(always)]
    pub fn tcaned(&self) -> TcanedR {
        TcanedR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrUsbhostMaskedSpec;
impl crate::RegisterSpec for IntrUsbhostMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost_masked::R`](R) reader structure"]
impl crate::Readable for IntrUsbhostMaskedSpec {}
#[doc = "`reset()` method sets INTR_USBHOST_MASKED to value 0"]
impl crate::Resettable for IntrUsbhostMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
