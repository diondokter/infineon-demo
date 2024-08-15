#[doc = "Register `WDT_MATCH` reader"]
pub type R = crate::R<WdtMatchSpec>;
#[doc = "Register `WDT_MATCH` writer"]
pub type W = crate::W<WdtMatchSpec>;
#[doc = "Field `MATCH` reader - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
pub type MatchR = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
pub type MatchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IGNORE_BITS` reader - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
pub type IgnoreBitsR = crate::FieldReader;
#[doc = "Field `IGNORE_BITS` writer - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
pub type IgnoreBitsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub fn ignore_bits(&self) -> IgnoreBitsR {
        IgnoreBitsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MatchW<WdtMatchSpec> {
        MatchW::new(self, 0)
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    #[must_use]
    pub fn ignore_bits(&mut self) -> IgnoreBitsW<WdtMatchSpec> {
        IgnoreBitsW::new(self, 16)
    }
}
#[doc = "Watchdog Counter Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_match::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_match::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtMatchSpec;
impl crate::RegisterSpec for WdtMatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_match::R`](R) reader structure"]
impl crate::Readable for WdtMatchSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_match::W`](W) writer structure"]
impl crate::Writable for WdtMatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT_MATCH to value 0x1000"]
impl crate::Resettable for WdtMatchSpec {
    const RESET_VALUE: u32 = 0x1000;
}
