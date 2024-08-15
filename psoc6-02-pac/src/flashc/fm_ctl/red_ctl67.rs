#[doc = "Register `RED_CTL67` reader"]
pub type R = crate::R<RedCtl67Spec>;
#[doc = "Register `RED_CTL67` writer"]
pub type W = crate::W<RedCtl67Spec>;
#[doc = "Field `RED_ADDR_6` reader - Bad Row Pair Address for Sector 6"]
pub type RedAddr6R = crate::FieldReader;
#[doc = "Field `RED_ADDR_6` writer - Bad Row Pair Address for Sector 6"]
pub type RedAddr6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_6` reader - 1: Redundancy Enable for Sector 6"]
pub type RedEn6R = crate::BitReader;
#[doc = "Field `RED_EN_6` writer - 1: Redundancy Enable for Sector 6"]
pub type RedEn6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_7` reader - Bad Row Pair Address for Sector 7"]
pub type RedAddr7R = crate::FieldReader;
#[doc = "Field `RED_ADDR_7` writer - Bad Row Pair Address for Sector 7"]
pub type RedAddr7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_7` reader - 1: Redundancy Enable for Sector 7"]
pub type RedEn7R = crate::BitReader;
#[doc = "Field `RED_EN_7` writer - 1: Redundancy Enable for Sector 7"]
pub type RedEn7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 6"]
    #[inline(always)]
    pub fn red_addr_6(&self) -> RedAddr6R {
        RedAddr6R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 6"]
    #[inline(always)]
    pub fn red_en_6(&self) -> RedEn6R {
        RedEn6R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 7"]
    #[inline(always)]
    pub fn red_addr_7(&self) -> RedAddr7R {
        RedAddr7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 7"]
    #[inline(always)]
    pub fn red_en_7(&self) -> RedEn7R {
        RedEn7R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 6"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_6(&mut self) -> RedAddr6W<RedCtl67Spec> {
        RedAddr6W::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 6"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_6(&mut self) -> RedEn6W<RedCtl67Spec> {
        RedEn6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_7(&mut self) -> RedAddr7W<RedCtl67Spec> {
        RedAddr7W::new(self, 16)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_7(&mut self) -> RedEn7W<RedCtl67Spec> {
        RedEn7W::new(self, 24)
    }
}
#[doc = "Redundancy Control normal sectors 6,7\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedCtl67Spec;
impl crate::RegisterSpec for RedCtl67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl67::R`](R) reader structure"]
impl crate::Readable for RedCtl67Spec {}
#[doc = "`write(|w| ..)` method takes [`red_ctl67::W`](W) writer structure"]
impl crate::Writable for RedCtl67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RED_CTL67 to value 0"]
impl crate::Resettable for RedCtl67Spec {
    const RESET_VALUE: u32 = 0;
}
