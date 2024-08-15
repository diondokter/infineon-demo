#[doc = "Register `HOST_EP1_RW2_DR` reader"]
pub type R = crate::R<HostEp1Rw2DrSpec>;
#[doc = "Register `HOST_EP1_RW2_DR` writer"]
pub type W = crate::W<HostEp1Rw2DrSpec>;
#[doc = "Field `BFDT16` reader - Data Register for EP1 for 2-byte data"]
pub type Bfdt16R = crate::FieldReader<u16>;
#[doc = "Field `BFDT16` writer - Data Register for EP1 for 2-byte data"]
pub type Bfdt16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register for EP1 for 2-byte data"]
    #[inline(always)]
    pub fn bfdt16(&self) -> Bfdt16R {
        Bfdt16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP1 for 2-byte data"]
    #[inline(always)]
    #[must_use]
    pub fn bfdt16(&mut self) -> Bfdt16W<HostEp1Rw2DrSpec> {
        Bfdt16W::new(self, 0)
    }
}
#[doc = "Host Endpoint 1 Data 2-Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_rw2_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep1_rw2_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostEp1Rw2DrSpec;
impl crate::RegisterSpec for HostEp1Rw2DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep1_rw2_dr::R`](R) reader structure"]
impl crate::Readable for HostEp1Rw2DrSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ep1_rw2_dr::W`](W) writer structure"]
impl crate::Writable for HostEp1Rw2DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_EP1_RW2_DR to value 0"]
impl crate::Resettable for HostEp1Rw2DrSpec {
    const RESET_VALUE: u32 = 0;
}
