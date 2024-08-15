#[doc = "Register `INTR_RX_SET` reader"]
pub type R = crate::R<IntrRxSetSpec>;
#[doc = "Register `INTR_RX_SET` writer"]
pub type W = crate::W<IntrRxSetSpec>;
#[doc = "Field `TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TriggerR = crate::BitReader;
#[doc = "Field `TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_EMPTY` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type NotEmptyR = crate::BitReader;
#[doc = "Field `NOT_EMPTY` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type NotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type FullR = crate::BitReader;
#[doc = "Field `FULL` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type FullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type BlockedR = crate::BitReader;
#[doc = "Field `BLOCKED` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type BlockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_ERROR` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type FrameErrorR = crate::BitReader;
#[doc = "Field `FRAME_ERROR` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type FrameErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERROR` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type ParityErrorR = crate::BitReader;
#[doc = "Field `PARITY_ERROR` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type ParityErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAUD_DETECT` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type BaudDetectR = crate::BitReader;
#[doc = "Field `BAUD_DETECT` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type BaudDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_DETECT` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type BreakDetectR = crate::BitReader;
#[doc = "Field `BREAK_DETECT` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type BreakDetectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn not_empty(&self) -> NotEmptyR {
        NotEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn blocked(&self) -> BlockedR {
        BlockedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn frame_error(&self) -> FrameErrorR {
        FrameErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn parity_error(&self) -> ParityErrorR {
        ParityErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BaudDetectR {
        BaudDetectR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn break_detect(&self) -> BreakDetectR {
        BreakDetectR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TriggerW<IntrRxSetSpec> {
        TriggerW::new(self, 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn not_empty(&mut self) -> NotEmptyW<IntrRxSetSpec> {
        NotEmptyW::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FullW<IntrRxSetSpec> {
        FullW::new(self, 3)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OverflowW<IntrRxSetSpec> {
        OverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UnderflowW<IntrRxSetSpec> {
        UnderflowW::new(self, 6)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BlockedW<IntrRxSetSpec> {
        BlockedW::new(self, 7)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_error(&mut self) -> FrameErrorW<IntrRxSetSpec> {
        FrameErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn parity_error(&mut self) -> ParityErrorW<IntrRxSetSpec> {
        ParityErrorW::new(self, 9)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn baud_detect(&mut self) -> BaudDetectW<IntrRxSetSpec> {
        BaudDetectW::new(self, 10)
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn break_detect(&mut self) -> BreakDetectW<IntrRxSetSpec> {
        BreakDetectW::new(self, 11)
    }
}
#[doc = "Receiver interrupt set request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_rx_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_rx_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRxSetSpec;
impl crate::RegisterSpec for IntrRxSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rx_set::R`](R) reader structure"]
impl crate::Readable for IntrRxSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_rx_set::W`](W) writer structure"]
impl crate::Writable for IntrRxSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_RX_SET to value 0"]
impl crate::Resettable for IntrRxSetSpec {
    const RESET_VALUE: u32 = 0;
}
