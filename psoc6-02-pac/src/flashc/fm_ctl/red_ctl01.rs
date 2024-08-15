#[doc = "Register `RED_CTL01` reader"]
pub type R = crate::R<RedCtl01Spec>;
#[doc = "Register `RED_CTL01` writer"]
pub type W = crate::W<RedCtl01Spec>;
#[doc = "Field `RED_ADDR_0` reader - Bad Row Pair Address for Sector 0"]
pub type RedAddr0R = crate::FieldReader;
#[doc = "Field `RED_ADDR_0` writer - Bad Row Pair Address for Sector 0"]
pub type RedAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_0` reader - 1: Redundancy Enable for Sector 0"]
pub type RedEn0R = crate::BitReader;
#[doc = "Field `RED_EN_0` writer - 1: Redundancy Enable for Sector 0"]
pub type RedEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_1` reader - Bad Row Pair Address for Sector 1"]
pub type RedAddr1R = crate::FieldReader;
#[doc = "Field `RED_ADDR_1` writer - Bad Row Pair Address for Sector 1"]
pub type RedAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_1` reader - 1: Redundancy Enable for Sector 1"]
pub type RedEn1R = crate::BitReader;
#[doc = "Field `RED_EN_1` writer - 1: Redundancy Enable for Sector 1"]
pub type RedEn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub fn red_addr_0(&self) -> RedAddr0R {
        RedAddr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub fn red_en_0(&self) -> RedEn0R {
        RedEn0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub fn red_addr_1(&self) -> RedAddr1R {
        RedAddr1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub fn red_en_1(&self) -> RedEn1R {
        RedEn1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_0(&mut self) -> RedAddr0W<RedCtl01Spec> {
        RedAddr0W::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_0(&mut self) -> RedEn0W<RedCtl01Spec> {
        RedEn0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_1(&mut self) -> RedAddr1W<RedCtl01Spec> {
        RedAddr1W::new(self, 16)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_1(&mut self) -> RedEn1W<RedCtl01Spec> {
        RedEn1W::new(self, 24)
    }
}
#[doc = "Redundancy Control normal sectors 0,1\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedCtl01Spec;
impl crate::RegisterSpec for RedCtl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl01::R`](R) reader structure"]
impl crate::Readable for RedCtl01Spec {}
#[doc = "`write(|w| ..)` method takes [`red_ctl01::W`](W) writer structure"]
impl crate::Writable for RedCtl01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RED_CTL01 to value 0"]
impl crate::Resettable for RedCtl01Spec {
    const RESET_VALUE: u32 = 0;
}
