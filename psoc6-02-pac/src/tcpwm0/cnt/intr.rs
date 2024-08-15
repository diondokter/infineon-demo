#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `TC` reader - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC_MATCH` reader - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type CcMatchR = crate::BitReader;
#[doc = "Field `CC_MATCH` writer - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type CcMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn cc_match(&self) -> CcMatchR {
        CcMatchR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<IntrSpec> {
        TcW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn cc_match(&mut self) -> CcMatchW<IntrSpec> {
        CcMatchW::new(self, 1)
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
