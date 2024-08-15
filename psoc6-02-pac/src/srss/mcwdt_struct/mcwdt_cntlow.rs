#[doc = "Register `MCWDT_CNTLOW` reader"]
pub type R = crate::R<McwdtCntlowSpec>;
#[doc = "Register `MCWDT_CNTLOW` writer"]
pub type W = crate::W<McwdtCntlowSpec>;
#[doc = "Field `WDT_CTR0` reader - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
pub type WdtCtr0R = crate::FieldReader<u16>;
#[doc = "Field `WDT_CTR0` writer - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
pub type WdtCtr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WDT_CTR1` reader - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WdtCtr1R = crate::FieldReader<u16>;
#[doc = "Field `WDT_CTR1` writer - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WdtCtr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub fn wdt_ctr0(&self) -> WdtCtr0R {
        WdtCtr0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr1(&self) -> WdtCtr1R {
        WdtCtr1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctr0(&mut self) -> WdtCtr0W<McwdtCntlowSpec> {
        WdtCtr0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctr1(&mut self) -> WdtCtr1W<McwdtCntlowSpec> {
        WdtCtr1W::new(self, 16)
    }
}
#[doc = "Multi-Counter Watchdog Sub-counters 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_cntlow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_cntlow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtCntlowSpec;
impl crate::RegisterSpec for McwdtCntlowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_cntlow::R`](R) reader structure"]
impl crate::Readable for McwdtCntlowSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_cntlow::W`](W) writer structure"]
impl crate::Writable for McwdtCntlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_CNTLOW to value 0"]
impl crate::Resettable for McwdtCntlowSpec {
    const RESET_VALUE: u32 = 0;
}
