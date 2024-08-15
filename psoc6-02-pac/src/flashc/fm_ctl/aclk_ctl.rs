#[doc = "Register `ACLK_CTL` writer"]
pub type W = crate::W<AclkCtlSpec>;
#[doc = "Field `ACLK_GEN` writer - A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
pub type AclkGenW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gen(&mut self) -> AclkGenW<AclkCtlSpec> {
        AclkGenW::new(self, 0)
    }
}
#[doc = "MPCON clock\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aclk_ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AclkCtlSpec;
impl crate::RegisterSpec for AclkCtlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aclk_ctl::W`](W) writer structure"]
impl crate::Writable for AclkCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACLK_CTL to value 0"]
impl crate::Resettable for AclkCtlSpec {
    const RESET_VALUE: u32 = 0;
}
