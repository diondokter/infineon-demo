#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `TX_TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxTriggerR = crate::BitReader;
#[doc = "Field `TX_TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NOT_FULL` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxNotFullR = crate::BitReader;
#[doc = "Field `TX_NOT_FULL` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxNotFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxOverflowR = crate::BitReader;
#[doc = "Field `TX_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxUnderflowR = crate::BitReader;
#[doc = "Field `TX_UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_WD` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxWdR = crate::BitReader;
#[doc = "Field `TX_WD` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxWdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxTriggerR = crate::BitReader;
#[doc = "Field `RX_TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxNotEmptyR = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxFullR = crate::BitReader;
#[doc = "Field `RX_FULL` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxOverflowR = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxUnderflowR = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_WD` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxWdR = crate::BitReader;
#[doc = "Field `RX_WD` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxWdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_trigger(&self) -> TxTriggerR {
        TxTriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_not_full(&self) -> TxNotFullR {
        TxNotFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_overflow(&self) -> TxOverflowR {
        TxOverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TxUnderflowR {
        TxUnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_wd(&self) -> TxWdR {
        TxWdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RxTriggerR {
        RxTriggerR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RxNotEmptyR {
        RxNotEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RxOverflowR {
        RxOverflowR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RxUnderflowR {
        RxUnderflowR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_wd(&self) -> RxWdR {
        RxWdR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_trigger(&mut self) -> TxTriggerW<IntrSetSpec> {
        TxTriggerW::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_not_full(&mut self) -> TxNotFullW<IntrSetSpec> {
        TxNotFullW::new(self, 1)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<IntrSetSpec> {
        TxEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_overflow(&mut self) -> TxOverflowW<IntrSetSpec> {
        TxOverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_underflow(&mut self) -> TxUnderflowW<IntrSetSpec> {
        TxUnderflowW::new(self, 6)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wd(&mut self) -> TxWdW<IntrSetSpec> {
        TxWdW::new(self, 8)
    }
    #[doc = "Bit 16 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_trigger(&mut self) -> RxTriggerW<IntrSetSpec> {
        RxTriggerW::new(self, 16)
    }
    #[doc = "Bit 18 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_not_empty(&mut self) -> RxNotEmptyW<IntrSetSpec> {
        RxNotEmptyW::new(self, 18)
    }
    #[doc = "Bit 19 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RxFullW<IntrSetSpec> {
        RxFullW::new(self, 19)
    }
    #[doc = "Bit 21 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow(&mut self) -> RxOverflowW<IntrSetSpec> {
        RxOverflowW::new(self, 21)
    }
    #[doc = "Bit 22 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_underflow(&mut self) -> RxUnderflowW<IntrSetSpec> {
        RxUnderflowW::new(self, 22)
    }
    #[doc = "Bit 24 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wd(&mut self) -> RxWdW<IntrSetSpec> {
        RxWdW::new(self, 24)
    }
}
#[doc = "Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSetSpec;
impl crate::RegisterSpec for IntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for IntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for IntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for IntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
