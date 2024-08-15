#[doc = "Register `FM_ADDR` reader"]
pub type R = crate::R<FmAddrSpec>;
#[doc = "Register `FM_ADDR` writer"]
pub type W = crate::W<FmAddrSpec>;
#[doc = "Field `RA` reader - Row address."]
pub type RaR = crate::FieldReader<u16>;
#[doc = "Field `RA` writer - Row address."]
pub type RaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BA` reader - Bank address."]
pub type BaR = crate::FieldReader;
#[doc = "Field `BA` writer - Bank address."]
pub type BaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AXA` reader - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
pub type AxaR = crate::BitReader;
#[doc = "Field `AXA` writer - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
pub type AxaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&self) -> BaR {
        BaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&self) -> AxaR {
        AxaR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<FmAddrSpec> {
        RaW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    #[must_use]
    pub fn ba(&mut self) -> BaW<FmAddrSpec> {
        BaW::new(self, 16)
    }
    #[doc = "Bit 24 - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
    #[inline(always)]
    #[must_use]
    pub fn axa(&mut self) -> AxaW<FmAddrSpec> {
        AxaW::new(self, 24)
    }
}
#[doc = "Flash macro address\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmAddrSpec;
impl crate::RegisterSpec for FmAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_addr::R`](R) reader structure"]
impl crate::Readable for FmAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`fm_addr::W`](W) writer structure"]
impl crate::Writable for FmAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM_ADDR to value 0"]
impl crate::Resettable for FmAddrSpec {
    const RESET_VALUE: u32 = 0;
}
