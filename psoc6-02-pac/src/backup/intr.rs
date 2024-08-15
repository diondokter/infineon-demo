#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `ALARM1` reader - Alarm 1 Interrupt"]
pub type Alarm1R = crate::BitReader;
#[doc = "Field `ALARM1` writer - Alarm 1 Interrupt"]
pub type Alarm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM2` reader - Alarm 2 Interrupt"]
pub type Alarm2R = crate::BitReader;
#[doc = "Field `ALARM2` writer - Alarm 2 Interrupt"]
pub type Alarm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTURY` reader - Century overflow interrupt"]
pub type CenturyR = crate::BitReader;
#[doc = "Field `CENTURY` writer - Century overflow interrupt"]
pub type CenturyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alarm 1 Interrupt"]
    #[inline(always)]
    pub fn alarm1(&self) -> Alarm1R {
        Alarm1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm 2 Interrupt"]
    #[inline(always)]
    pub fn alarm2(&self) -> Alarm2R {
        Alarm2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Century overflow interrupt"]
    #[inline(always)]
    pub fn century(&self) -> CenturyR {
        CenturyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> Alarm1W<IntrSpec> {
        Alarm1W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm 2 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn alarm2(&mut self) -> Alarm2W<IntrSpec> {
        Alarm2W::new(self, 1)
    }
    #[doc = "Bit 2 - Century overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn century(&mut self) -> CenturyW<IntrSpec> {
        CenturyW::new(self, 2)
    }
}
#[doc = "Interrupt request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
