#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `ALARM1` reader - Mask bit for corresponding bit in interrupt request register."]
pub type Alarm1R = crate::BitReader;
#[doc = "Field `ALARM1` writer - Mask bit for corresponding bit in interrupt request register."]
pub type Alarm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM2` reader - Mask bit for corresponding bit in interrupt request register."]
pub type Alarm2R = crate::BitReader;
#[doc = "Field `ALARM2` writer - Mask bit for corresponding bit in interrupt request register."]
pub type Alarm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTURY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type CenturyR = crate::BitReader;
#[doc = "Field `CENTURY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type CenturyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn alarm1(&self) -> Alarm1R {
        Alarm1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn alarm2(&self) -> Alarm2R {
        Alarm2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn century(&self) -> CenturyR {
        CenturyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> Alarm1W<IntrMaskSpec> {
        Alarm1W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn alarm2(&mut self) -> Alarm2W<IntrMaskSpec> {
        Alarm2W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn century(&mut self) -> CenturyW<IntrMaskSpec> {
        CenturyW::new(self, 2)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
