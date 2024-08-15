#[doc = "Register `MCWDT_MATCH` reader"]
pub type R = crate::R<McwdtMatchSpec>;
#[doc = "Register `MCWDT_MATCH` writer"]
pub type W = crate::W<McwdtMatchSpec>;
#[doc = "Field `WDT_MATCH0` reader - Match value for sub-counter 0 of this MCWDT"]
pub type WdtMatch0R = crate::FieldReader<u16>;
#[doc = "Field `WDT_MATCH0` writer - Match value for sub-counter 0 of this MCWDT"]
pub type WdtMatch0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WDT_MATCH1` reader - Match value for sub-counter 1 of this MCWDT"]
pub type WdtMatch1R = crate::FieldReader<u16>;
#[doc = "Field `WDT_MATCH1` writer - Match value for sub-counter 1 of this MCWDT"]
pub type WdtMatch1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match0(&self) -> WdtMatch0R {
        WdtMatch0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match1(&self) -> WdtMatch1R {
        WdtMatch1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match0(&mut self) -> WdtMatch0W<McwdtMatchSpec> {
        WdtMatch0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_match1(&mut self) -> WdtMatch1W<McwdtMatchSpec> {
        WdtMatch1W::new(self, 16)
    }
}
#[doc = "Multi-Counter Watchdog Counter Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_match::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_match::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtMatchSpec;
impl crate::RegisterSpec for McwdtMatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_match::R`](R) reader structure"]
impl crate::Readable for McwdtMatchSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_match::W`](W) writer structure"]
impl crate::Writable for McwdtMatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_MATCH to value 0"]
impl crate::Resettable for McwdtMatchSpec {
    const RESET_VALUE: u32 = 0;
}
