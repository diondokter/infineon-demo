#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `TX_TRIGGER` reader - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
pub type TxTriggerR = crate::BitReader;
#[doc = "Field `TX_TRIGGER` writer - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
pub type TxTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NOT_FULL` reader - TX FIFO is not full."]
pub type TxNotFullR = crate::BitReader;
#[doc = "Field `TX_NOT_FULL` writer - TX FIFO is not full."]
pub type TxNotFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - TX FIFO is empty; i.e. it has 0 entries."]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - TX FIFO is empty; i.e. it has 0 entries."]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OVERFLOW` reader - Attempt to write to a full TX FIFO."]
pub type TxOverflowR = crate::BitReader;
#[doc = "Field `TX_OVERFLOW` writer - Attempt to write to a full TX FIFO."]
pub type TxOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_UNDERFLOW` reader - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
pub type TxUnderflowR = crate::BitReader;
#[doc = "Field `TX_UNDERFLOW` writer - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
pub type TxUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_WD` reader - Triggers (sets to '1') when the Tx watchdog event occurs."]
pub type TxWdR = crate::BitReader;
#[doc = "Field `TX_WD` writer - Triggers (sets to '1') when the Tx watchdog event occurs."]
pub type TxWdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRIGGER` reader - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
pub type RxTriggerR = crate::BitReader;
#[doc = "Field `RX_TRIGGER` writer - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
pub type RxTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY` reader - RX FIFO is not empty."]
pub type RxNotEmptyR = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` writer - RX FIFO is not empty."]
pub type RxNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - RX FIFO is full."]
pub type RxFullR = crate::BitReader;
#[doc = "Field `RX_FULL` writer - RX FIFO is full."]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW` reader - Attempt to write to a full RX FIFO."]
pub type RxOverflowR = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` writer - Attempt to write to a full RX FIFO."]
pub type RxOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UNDERFLOW` reader - Attempt to read from an empty RX FIFO."]
pub type RxUnderflowR = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` writer - Attempt to read from an empty RX FIFO."]
pub type RxUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_WD` reader - Triggers (sets to '1') when the Rx watchdog event occurs."]
pub type RxWdR = crate::BitReader;
#[doc = "Field `RX_WD` writer - Triggers (sets to '1') when the Rx watchdog event occurs."]
pub type RxWdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
    #[inline(always)]
    pub fn tx_trigger(&self) -> TxTriggerR {
        TxTriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO is not full."]
    #[inline(always)]
    pub fn tx_not_full(&self) -> TxNotFullR {
        TxNotFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO."]
    #[inline(always)]
    pub fn tx_overflow(&self) -> TxOverflowR {
        TxOverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TxUnderflowR {
        TxUnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Triggers (sets to '1') when the Tx watchdog event occurs."]
    #[inline(always)]
    pub fn tx_wd(&self) -> TxWdR {
        TxWdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RxTriggerR {
        RxTriggerR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - RX FIFO is not empty."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RxNotEmptyR {
        RxNotEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RX FIFO is full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RxOverflowR {
        RxOverflowR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RxUnderflowR {
        RxUnderflowR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Triggers (sets to '1') when the Rx watchdog event occurs."]
    #[inline(always)]
    pub fn rx_wd(&self) -> RxWdR {
        RxWdR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
    #[inline(always)]
    #[must_use]
    pub fn tx_trigger(&mut self) -> TxTriggerW<IntrSpec> {
        TxTriggerW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO is not full."]
    #[inline(always)]
    #[must_use]
    pub fn tx_not_full(&mut self) -> TxNotFullW<IntrSpec> {
        TxNotFullW::new(self, 1)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<IntrSpec> {
        TxEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_overflow(&mut self) -> TxOverflowW<IntrSpec> {
        TxOverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tx_underflow(&mut self) -> TxUnderflowW<IntrSpec> {
        TxUnderflowW::new(self, 6)
    }
    #[doc = "Bit 8 - Triggers (sets to '1') when the Tx watchdog event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wd(&mut self) -> TxWdW<IntrSpec> {
        TxWdW::new(self, 8)
    }
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
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
    #[doc = "Bit 19 - RX FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RxFullW<IntrSpec> {
        RxFullW::new(self, 19)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow(&mut self) -> RxOverflowW<IntrSpec> {
        RxOverflowW::new(self, 21)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_underflow(&mut self) -> RxUnderflowW<IntrSpec> {
        RxUnderflowW::new(self, 22)
    }
    #[doc = "Bit 24 - Triggers (sets to '1') when the Rx watchdog event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wd(&mut self) -> RxWdW<IntrSpec> {
        RxWdW::new(self, 24)
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
