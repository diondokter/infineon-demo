#[doc = "Register `RED_CTL23` reader"]
pub type R = crate::R<RedCtl23Spec>;
#[doc = "Register `RED_CTL23` writer"]
pub type W = crate::W<RedCtl23Spec>;
#[doc = "Field `RED_ADDR_2` reader - Bad Row Pair Address for Sector 2"]
pub type RedAddr2R = crate::FieldReader;
#[doc = "Field `RED_ADDR_2` writer - Bad Row Pair Address for Sector 2"]
pub type RedAddr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_2` reader - 1: Redundancy Enable for Sector 2"]
pub type RedEn2R = crate::BitReader;
#[doc = "Field `RED_EN_2` writer - 1: Redundancy Enable for Sector 2"]
pub type RedEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_3` reader - Bad Row Pair Address for Sector 3"]
pub type RedAddr3R = crate::FieldReader;
#[doc = "Field `RED_ADDR_3` writer - Bad Row Pair Address for Sector 3"]
pub type RedAddr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_3` reader - 1: Redundancy Enable for Sector 3"]
pub type RedEn3R = crate::BitReader;
#[doc = "Field `RED_EN_3` writer - 1: Redundancy Enable for Sector 3"]
pub type RedEn3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    pub fn red_addr_2(&self) -> RedAddr2R {
        RedAddr2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub fn red_en_2(&self) -> RedEn2R {
        RedEn2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    pub fn red_addr_3(&self) -> RedAddr3R {
        RedAddr3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub fn red_en_3(&self) -> RedEn3R {
        RedEn3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_2(&mut self) -> RedAddr2W<RedCtl23Spec> {
        RedAddr2W::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 2"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_2(&mut self) -> RedEn2W<RedCtl23Spec> {
        RedEn2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_3(&mut self) -> RedAddr3W<RedCtl23Spec> {
        RedAddr3W::new(self, 16)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 3"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_3(&mut self) -> RedEn3W<RedCtl23Spec> {
        RedEn3W::new(self, 24)
    }
}
#[doc = "Redundancy Control normal sectors 2,3\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedCtl23Spec;
impl crate::RegisterSpec for RedCtl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl23::R`](R) reader structure"]
impl crate::Readable for RedCtl23Spec {}
#[doc = "`write(|w| ..)` method takes [`red_ctl23::W`](W) writer structure"]
impl crate::Writable for RedCtl23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RED_CTL23 to value 0"]
impl crate::Resettable for RedCtl23Spec {
    const RESET_VALUE: u32 = 0;
}
