#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `COMP0` reader - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP0` writer - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` reader - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COMP1` writer - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> Comp0W<IntrSpec> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> Comp1W<IntrSpec> {
        Comp1W::new(self, 1)
    }
}
#[doc = "LPCOMP Interrupt request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
