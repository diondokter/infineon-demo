#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `RX_TRIGGER` reader - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
pub type RxTriggerR = crate::BitReader;
#[doc = "Field `RX_TRIGGER` writer - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
pub type RxTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY` reader - RX FIFO is not empty."]
pub type RxNotEmptyR = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` writer - RX FIFO is not empty."]
pub type RxNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW` reader - Attempt to write to a full RX FIFO"]
pub type RxOverflowR = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` writer - Attempt to write to a full RX FIFO"]
pub type RxOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UNDERFLOW` reader - Attempt to read from an empty RX FIFO"]
pub type RxUnderflowR = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` writer - Attempt to read from an empty RX FIFO"]
pub type RxUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RxTriggerR {
        RxTriggerR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - RX FIFO is not empty."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RxNotEmptyR {
        RxNotEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO"]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RxOverflowR {
        RxOverflowR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO"]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RxUnderflowR {
        RxUnderflowR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn rx_trigger(&mut self) -> RxTriggerW<IntrSpec> {
        RxTriggerW::new(self, 16)
    }
    #[doc = "Bit 18 - RX FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn rx_not_empty(&mut self) -> RxNotEmptyW<IntrSpec> {
        RxNotEmptyW::new(self, 18)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow(&mut self) -> RxOverflowW<IntrSpec> {
        RxOverflowW::new(self, 21)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_underflow(&mut self) -> RxUnderflowW<IntrSpec> {
        RxUnderflowW::new(self, 22)
    }
}
#[doc = "Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
