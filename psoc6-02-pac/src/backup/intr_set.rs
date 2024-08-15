#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `ALARM1` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type Alarm1R = crate::BitReader;
#[doc = "Field `ALARM1` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type Alarm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM2` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type Alarm2R = crate::BitReader;
#[doc = "Field `ALARM2` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type Alarm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTURY` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type CenturyR = crate::BitReader;
#[doc = "Field `CENTURY` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type CenturyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn alarm1(&self) -> Alarm1R {
        Alarm1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn alarm2(&self) -> Alarm2R {
        Alarm2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn century(&self) -> CenturyR {
        CenturyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> Alarm1W<IntrSetSpec> {
        Alarm1W::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn alarm2(&mut self) -> Alarm2W<IntrSetSpec> {
        Alarm2W::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn century(&mut self) -> CenturyW<IntrSetSpec> {
        CenturyW::new(self, 2)
    }
}
#[doc = "Interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
