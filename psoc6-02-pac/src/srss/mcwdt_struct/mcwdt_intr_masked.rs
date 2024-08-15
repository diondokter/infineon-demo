#[doc = "Register `MCWDT_INTR_MASKED` reader"]
pub type R = crate::R<McwdtIntrMaskedSpec>;
#[doc = "Field `MCWDT_INT0` reader - Logical and of corresponding request and mask bits."]
pub type McwdtInt0R = crate::BitReader;
#[doc = "Field `MCWDT_INT1` reader - Logical and of corresponding request and mask bits."]
pub type McwdtInt1R = crate::BitReader;
#[doc = "Field `MCWDT_INT2` reader - Logical and of corresponding request and mask bits."]
pub type McwdtInt2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> McwdtInt0R {
        McwdtInt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> McwdtInt1R {
        McwdtInt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> McwdtInt2R {
        McwdtInt2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtIntrMaskedSpec;
impl crate::RegisterSpec for McwdtIntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_intr_masked::R`](R) reader structure"]
impl crate::Readable for McwdtIntrMaskedSpec {}
#[doc = "`reset()` method sets MCWDT_INTR_MASKED to value 0"]
impl crate::Resettable for McwdtIntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
