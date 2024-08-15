#[doc = "Register `RED_CTL_SM01` reader"]
pub type R = crate::R<RedCtlSm01Spec>;
#[doc = "Register `RED_CTL_SM01` writer"]
pub type W = crate::W<RedCtlSm01Spec>;
#[doc = "Field `RED_ADDR_SM0` reader - Bad Row Pair Address for Special Sector 0"]
pub type RedAddrSm0R = crate::FieldReader;
#[doc = "Field `RED_ADDR_SM0` writer - Bad Row Pair Address for Special Sector 0"]
pub type RedAddrSm0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_SM0` reader - Redundancy Enable for Special Sector 0"]
pub type RedEnSm0R = crate::BitReader;
#[doc = "Field `RED_EN_SM0` writer - Redundancy Enable for Special Sector 0"]
pub type RedEnSm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_SM1` reader - Bad Row Pair Address for Special Sector 1"]
pub type RedAddrSm1R = crate::FieldReader;
#[doc = "Field `RED_ADDR_SM1` writer - Bad Row Pair Address for Special Sector 1"]
pub type RedAddrSm1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_SM1` reader - Redundancy Enable for Special Sector 1"]
pub type RedEnSm1R = crate::BitReader;
#[doc = "Field `RED_EN_SM1` writer - Redundancy Enable for Special Sector 1"]
pub type RedEnSm1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub fn red_addr_sm0(&self) -> RedAddrSm0R {
        RedAddrSm0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub fn red_en_sm0(&self) -> RedEnSm0R {
        RedEnSm0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub fn red_addr_sm1(&self) -> RedAddrSm1R {
        RedAddrSm1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub fn red_en_sm1(&self) -> RedEnSm1R {
        RedEnSm1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_sm0(&mut self) -> RedAddrSm0W<RedCtlSm01Spec> {
        RedAddrSm0W::new(self, 0)
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_sm0(&mut self) -> RedEnSm0W<RedCtlSm01Spec> {
        RedEnSm0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_sm1(&mut self) -> RedAddrSm1W<RedCtlSm01Spec> {
        RedAddrSm1W::new(self, 16)
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_sm1(&mut self) -> RedEnSm1W<RedCtlSm01Spec> {
        RedEnSm1W::new(self, 24)
    }
}
#[doc = "Redundancy Control special sectors 0,1\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl_sm01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl_sm01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedCtlSm01Spec;
impl crate::RegisterSpec for RedCtlSm01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl_sm01::R`](R) reader structure"]
impl crate::Readable for RedCtlSm01Spec {}
#[doc = "`write(|w| ..)` method takes [`red_ctl_sm01::W`](W) writer structure"]
impl crate::Writable for RedCtlSm01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RED_CTL_SM01 to value 0"]
impl crate::Resettable for RedCtlSm01Spec {
    const RESET_VALUE: u32 = 0;
}
