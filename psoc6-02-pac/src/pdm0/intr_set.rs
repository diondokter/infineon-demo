#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `RX_TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxTriggerR = crate::BitReader;
#[doc = "Field `RX_TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxNotEmptyR = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxOverflowR = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxUnderflowR = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
}
impl W {
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
