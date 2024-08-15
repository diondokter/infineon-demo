#[doc = "Register `FAST_CA_CMD` reader"]
pub type R = crate::R<FastCaCmdSpec>;
#[doc = "Register `FAST_CA_CMD` writer"]
pub type W = crate::W<FastCaCmdSpec>;
#[doc = "Field `INV` reader - See SLOW_CA_CMD.INV."]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - See SLOW_CA_CMD.INV."]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - See SLOW_CA_CMD.INV."]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See SLOW_CA_CMD.INV."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<FastCaCmdSpec> {
        InvW::new(self, 0)
    }
}
#[doc = "Fast cache command\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_ca_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_ca_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FastCaCmdSpec;
impl crate::RegisterSpec for FastCaCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fast_ca_cmd::R`](R) reader structure"]
impl crate::Readable for FastCaCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`fast_ca_cmd::W`](W) writer structure"]
impl crate::Writable for FastCaCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAST_CA_CMD to value 0"]
impl crate::Resettable for FastCaCmdSpec {
    const RESET_VALUE: u32 = 0;
}
