#[doc = "Register `ARB_RW1_DR` reader"]
pub type R = crate::R<ArbRw1DrSpec>;
#[doc = "Register `ARB_RW1_DR` writer"]
pub type W = crate::W<ArbRw1DrSpec>;
#[doc = "Field `DR` reader - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DrR = crate::FieldReader;
#[doc = "Field `DR` writer - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<ArbRw1DrSpec> {
        DrW::new(self, 0)
    }
}
#[doc = "Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw1DrSpec;
impl crate::RegisterSpec for ArbRw1DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw1_dr::R`](R) reader structure"]
impl crate::Readable for ArbRw1DrSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw1_dr::W`](W) writer structure"]
impl crate::Writable for ArbRw1DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW1_DR to value 0"]
impl crate::Resettable for ArbRw1DrSpec {
    const RESET_VALUE: u32 = 0;
}
