#[doc = "Register `SLOW_CA_CMD` reader"]
pub type R = crate::R<SlowCaCmdSpec>;
#[doc = "Register `SLOW_CA_CMD` writer"]
pub type W = crate::W<SlowCaCmdSpec>;
#[doc = "Field `INV` reader - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<SlowCaCmdSpec> {
        InvW::new(self, 0)
    }
}
#[doc = "Slow cache command\n\nYou can [`read`](crate::Reg::read) this register and get [`slow_ca_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slow_ca_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlowCaCmdSpec;
impl crate::RegisterSpec for SlowCaCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slow_ca_cmd::R`](R) reader structure"]
impl crate::Readable for SlowCaCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`slow_ca_cmd::W`](W) writer structure"]
impl crate::Writable for SlowCaCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLOW_CA_CMD to value 0"]
impl crate::Resettable for SlowCaCmdSpec {
    const RESET_VALUE: u32 = 0;
}
