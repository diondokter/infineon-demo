#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `COMP0_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type Comp0MaskR = crate::BitReader;
#[doc = "Field `COMP0_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type Comp0MaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type Comp1MaskR = crate::BitReader;
#[doc = "Field `COMP1_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type Comp1MaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn comp0_mask(&self) -> Comp0MaskR {
        Comp0MaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn comp1_mask(&self) -> Comp1MaskR {
        Comp1MaskR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn comp0_mask(&mut self) -> Comp0MaskW<IntrMaskSpec> {
        Comp0MaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn comp1_mask(&mut self) -> Comp1MaskW<IntrMaskSpec> {
        Comp1MaskW::new(self, 1)
    }
}
#[doc = "LPCOMP Interrupt request mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
