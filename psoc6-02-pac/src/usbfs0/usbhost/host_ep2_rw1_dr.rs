#[doc = "Register `HOST_EP2_RW1_DR` reader"]
pub type R = crate::R<HostEp2Rw1DrSpec>;
#[doc = "Register `HOST_EP2_RW1_DR` writer"]
pub type W = crate::W<HostEp2Rw1DrSpec>;
#[doc = "Field `BFDT8` reader - Data Register for EP2 for 1-byte data."]
pub type Bfdt8R = crate::FieldReader;
#[doc = "Field `BFDT8` writer - Data Register for EP2 for 1-byte data."]
pub type Bfdt8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Register for EP2 for 1-byte data."]
    #[inline(always)]
    pub fn bfdt8(&self) -> Bfdt8R {
        Bfdt8R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP2 for 1-byte data."]
    #[inline(always)]
    #[must_use]
    pub fn bfdt8(&mut self) -> Bfdt8W<HostEp2Rw1DrSpec> {
        Bfdt8W::new(self, 0)
    }
}
#[doc = "Host Endpoint 2 Data 1-Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep2_rw1_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep2_rw1_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostEp2Rw1DrSpec;
impl crate::RegisterSpec for HostEp2Rw1DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep2_rw1_dr::R`](R) reader structure"]
impl crate::Readable for HostEp2Rw1DrSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ep2_rw1_dr::W`](W) writer structure"]
impl crate::Writable for HostEp2Rw1DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_EP2_RW1_DR to value 0"]
impl crate::Resettable for HostEp2Rw1DrSpec {
    const RESET_VALUE: u32 = 0;
}
