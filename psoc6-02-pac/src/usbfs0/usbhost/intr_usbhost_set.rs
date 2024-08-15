#[doc = "Register `INTR_USBHOST_SET` reader"]
pub type R = crate::R<IntrUsbhostSetSpec>;
#[doc = "Register `INTR_USBHOST_SET` writer"]
pub type W = crate::W<IntrUsbhostSetSpec>;
#[doc = "Field `SOFIRQS` reader - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type SofirqsR = crate::BitReader;
#[doc = "Field `SOFIRQS` writer - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type SofirqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRQS` reader - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type DirqsR = crate::BitReader;
#[doc = "Field `DIRQS` writer - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type DirqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNIRQS` reader - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CnnirqsR = crate::BitReader;
#[doc = "Field `CNNIRQS` writer - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CnnirqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIRQS` reader - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CmpirqsR = crate::BitReader;
#[doc = "Field `CMPIRQS` writer - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CmpirqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URIRQS` reader - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type UrirqsR = crate::BitReader;
#[doc = "Field `URIRQS` writer - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type UrirqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKIRQS` reader - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type RwkirqsR = crate::BitReader;
#[doc = "Field `RWKIRQS` writer - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type RwkirqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - N/A"]
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCANS` reader - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type TcansR = crate::BitReader;
#[doc = "Field `TCANS` writer - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type TcansW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&self) -> SofirqsR {
        SofirqsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&self) -> DirqsR {
        DirqsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&self) -> CnnirqsR {
        CnnirqsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&self) -> CmpirqsR {
        CmpirqsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&self) -> UrirqsR {
        UrirqsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&self) -> RwkirqsR {
        RwkirqsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&self) -> TcansR {
        TcansR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn sofirqs(&mut self) -> SofirqsW<IntrUsbhostSetSpec> {
        SofirqsW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn dirqs(&mut self) -> DirqsW<IntrUsbhostSetSpec> {
        DirqsW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cnnirqs(&mut self) -> CnnirqsW<IntrUsbhostSetSpec> {
        CnnirqsW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cmpirqs(&mut self) -> CmpirqsW<IntrUsbhostSetSpec> {
        CmpirqsW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn urirqs(&mut self) -> UrirqsW<IntrUsbhostSetSpec> {
        UrirqsW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn rwkirqs(&mut self) -> RwkirqsW<IntrUsbhostSetSpec> {
        RwkirqsW::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_6(&mut self) -> Rsvd6W<IntrUsbhostSetSpec> {
        Rsvd6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn tcans(&mut self) -> TcansW<IntrUsbhostSetSpec> {
        TcansW::new(self, 7)
    }
}
#[doc = "Interrupt USB Host Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_usbhost_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrUsbhostSetSpec;
impl crate::RegisterSpec for IntrUsbhostSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost_set::R`](R) reader structure"]
impl crate::Readable for IntrUsbhostSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_usbhost_set::W`](W) writer structure"]
impl crate::Writable for IntrUsbhostSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_USBHOST_SET to value 0"]
impl crate::Resettable for IntrUsbhostSetSpec {
    const RESET_VALUE: u32 = 0;
}
