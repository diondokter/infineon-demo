#[doc = "Register `INTR_TX_MASK` reader"]
pub type R = crate::R<IntrTxMaskSpec>;
#[doc = "Register `INTR_TX_MASK` writer"]
pub type W = crate::W<IntrTxMaskSpec>;
#[doc = "Field `TRIGGER` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TriggerR = crate::BitReader;
#[doc = "Field `TRIGGER` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_FULL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type NotFullR = crate::BitReader;
#[doc = "Field `NOT_FULL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type NotFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - Mask bit for corresponding bit in interrupt request register."]
pub type BlockedR = crate::BitReader;
#[doc = "Field `BLOCKED` writer - Mask bit for corresponding bit in interrupt request register."]
pub type BlockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_NACK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UartNackR = crate::BitReader;
#[doc = "Field `UART_NACK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UartNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_DONE` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UartDoneR = crate::BitReader;
#[doc = "Field `UART_DONE` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UartDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_ARB_LOST` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UartArbLostR = crate::BitReader;
#[doc = "Field `UART_ARB_LOST` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UartArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn not_full(&self) -> NotFullR {
        NotFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn blocked(&self) -> BlockedR {
        BlockedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UartNackR {
        UartNackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_done(&self) -> UartDoneR {
        UartDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UartArbLostR {
        UartArbLostR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TriggerW<IntrTxMaskSpec> {
        TriggerW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn not_full(&mut self) -> NotFullW<IntrTxMaskSpec> {
        NotFullW::new(self, 1)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<IntrTxMaskSpec> {
        EmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OverflowW<IntrTxMaskSpec> {
        OverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UnderflowW<IntrTxMaskSpec> {
        UnderflowW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BlockedW<IntrTxMaskSpec> {
        BlockedW::new(self, 7)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn uart_nack(&mut self) -> UartNackW<IntrTxMaskSpec> {
        UartNackW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn uart_done(&mut self) -> UartDoneW<IntrTxMaskSpec> {
        UartDoneW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn uart_arb_lost(&mut self) -> UartArbLostW<IntrTxMaskSpec> {
        UartArbLostW::new(self, 10)
    }
}
#[doc = "Transmitter interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_tx_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_tx_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrTxMaskSpec;
impl crate::RegisterSpec for IntrTxMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_tx_mask::R`](R) reader structure"]
impl crate::Readable for IntrTxMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_tx_mask::W`](W) writer structure"]
impl crate::Writable for IntrTxMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_TX_MASK to value 0"]
impl crate::Resettable for IntrTxMaskSpec {
    const RESET_VALUE: u32 = 0;
}
