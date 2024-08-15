#[doc = "Register `RED_CTL45` reader"]
pub type R = crate::R<RedCtl45Spec>;
#[doc = "Register `RED_CTL45` writer"]
pub type W = crate::W<RedCtl45Spec>;
#[doc = "Field `RED_ADDR_4` reader - Bad Row Pair Address for Sector 4"]
pub type RedAddr4R = crate::FieldReader;
#[doc = "Field `RED_ADDR_4` writer - Bad Row Pair Address for Sector 4"]
pub type RedAddr4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_4` reader - 1: Redundancy Enable for Sector 4"]
pub type RedEn4R = crate::BitReader;
#[doc = "Field `RED_EN_4` writer - 1: Redundancy Enable for Sector 4"]
pub type RedEn4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_5` reader - Bad Row Pair Address for Sector 5"]
pub type RedAddr5R = crate::FieldReader;
#[doc = "Field `RED_ADDR_5` writer - Bad Row Pair Address for Sector 5"]
pub type RedAddr5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_5` reader - 1: Redundancy Enable for Sector 5"]
pub type RedEn5R = crate::BitReader;
#[doc = "Field `RED_EN_5` writer - 1: Redundancy Enable for Sector 5"]
pub type RedEn5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    pub fn red_addr_4(&self) -> RedAddr4R {
        RedAddr4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    pub fn red_en_4(&self) -> RedEn4R {
        RedEn4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    pub fn red_addr_5(&self) -> RedAddr5R {
        RedAddr5R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    pub fn red_en_5(&self) -> RedEn5R {
        RedEn5R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_4(&mut self) -> RedAddr4W<RedCtl45Spec> {
        RedAddr4W::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_4(&mut self) -> RedEn4W<RedCtl45Spec> {
        RedEn4W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_5(&mut self) -> RedAddr5W<RedCtl45Spec> {
        RedAddr5W::new(self, 16)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_5(&mut self) -> RedEn5W<RedCtl45Spec> {
        RedEn5W::new(self, 24)
    }
}
#[doc = "Redundancy Control normal sectors 4,5\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedCtl45Spec;
impl crate::RegisterSpec for RedCtl45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl45::R`](R) reader structure"]
impl crate::Readable for RedCtl45Spec {}
#[doc = "`write(|w| ..)` method takes [`red_ctl45::W`](W) writer structure"]
impl crate::Writable for RedCtl45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RED_CTL45 to value 0"]
impl crate::Resettable for RedCtl45Spec {
    const RESET_VALUE: u32 = 0;
}
