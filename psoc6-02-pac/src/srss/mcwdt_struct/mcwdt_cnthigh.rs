#[doc = "Register `MCWDT_CNTHIGH` reader"]
pub type R = crate::R<McwdtCnthighSpec>;
#[doc = "Register `MCWDT_CNTHIGH` writer"]
pub type W = crate::W<McwdtCnthighSpec>;
#[doc = "Field `WDT_CTR2` reader - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WdtCtr2R = crate::FieldReader<u32>;
#[doc = "Field `WDT_CTR2` writer - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WdtCtr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr2(&self) -> WdtCtr2R {
        WdtCtr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctr2(&mut self) -> WdtCtr2W<McwdtCnthighSpec> {
        WdtCtr2W::new(self, 0)
    }
}
#[doc = "Multi-Counter Watchdog Sub-counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_cnthigh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_cnthigh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtCnthighSpec;
impl crate::RegisterSpec for McwdtCnthighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_cnthigh::R`](R) reader structure"]
impl crate::Readable for McwdtCnthighSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_cnthigh::W`](W) writer structure"]
impl crate::Writable for McwdtCnthighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_CNTHIGH to value 0"]
impl crate::Resettable for McwdtCnthighSpec {
    const RESET_VALUE: u32 = 0;
}
