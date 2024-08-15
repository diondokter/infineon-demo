#[doc = "Register `MCWDT_INTR_MASK` reader"]
pub type R = crate::R<McwdtIntrMaskSpec>;
#[doc = "Register `MCWDT_INTR_MASK` writer"]
pub type W = crate::W<McwdtIntrMaskSpec>;
#[doc = "Field `MCWDT_INT0` reader - Mask for sub-counter 0"]
pub type McwdtInt0R = crate::BitReader;
#[doc = "Field `MCWDT_INT0` writer - Mask for sub-counter 0"]
pub type McwdtInt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCWDT_INT1` reader - Mask for sub-counter 1"]
pub type McwdtInt1R = crate::BitReader;
#[doc = "Field `MCWDT_INT1` writer - Mask for sub-counter 1"]
pub type McwdtInt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCWDT_INT2` reader - Mask for sub-counter 2"]
pub type McwdtInt2R = crate::BitReader;
#[doc = "Field `MCWDT_INT2` writer - Mask for sub-counter 2"]
pub type McwdtInt2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask for sub-counter 0"]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> McwdtInt0R {
        McwdtInt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for sub-counter 1"]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> McwdtInt1R {
        McwdtInt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for sub-counter 2"]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> McwdtInt2R {
        McwdtInt2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for sub-counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int0(&mut self) -> McwdtInt0W<McwdtIntrMaskSpec> {
        McwdtInt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask for sub-counter 1"]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int1(&mut self) -> McwdtInt1W<McwdtIntrMaskSpec> {
        McwdtInt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Mask for sub-counter 2"]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int2(&mut self) -> McwdtInt2W<McwdtIntrMaskSpec> {
        McwdtInt2W::new(self, 2)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtIntrMaskSpec;
impl crate::RegisterSpec for McwdtIntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_intr_mask::R`](R) reader structure"]
impl crate::Readable for McwdtIntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_intr_mask::W`](W) writer structure"]
impl crate::Writable for McwdtIntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_INTR_MASK to value 0"]
impl crate::Resettable for McwdtIntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
