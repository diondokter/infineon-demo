#[doc = "Register `SL_ATT2` reader"]
pub type R = crate::R<SlAtt2Spec>;
#[doc = "Register `SL_ATT2` writer"]
pub type W = crate::W<SlAtt2Spec>;
#[doc = "Field `PC8_UR` reader - Protection context 8, user read enable."]
pub type Pc8UrR = crate::BitReader;
#[doc = "Field `PC8_UR` writer - Protection context 8, user read enable."]
pub type Pc8UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC8_UW` reader - Protection context 8, user write enable."]
pub type Pc8UwR = crate::BitReader;
#[doc = "Field `PC8_UW` writer - Protection context 8, user write enable."]
pub type Pc8UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC8_PR` reader - Protection context 8, privileged read enable."]
pub type Pc8PrR = crate::BitReader;
#[doc = "Field `PC8_PR` writer - Protection context 8, privileged read enable."]
pub type Pc8PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC8_PW` reader - Protection context 8, privileged write enable."]
pub type Pc8PwR = crate::BitReader;
#[doc = "Field `PC8_PW` writer - Protection context 8, privileged write enable."]
pub type Pc8PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC8_NS` reader - Protection context 8, non-secure."]
pub type Pc8NsR = crate::BitReader;
#[doc = "Field `PC8_NS` writer - Protection context 8, non-secure."]
pub type Pc8NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC9_UR` reader - Protection context 9, user read enable."]
pub type Pc9UrR = crate::BitReader;
#[doc = "Field `PC9_UR` writer - Protection context 9, user read enable."]
pub type Pc9UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC9_UW` reader - Protection context 9, user write enable."]
pub type Pc9UwR = crate::BitReader;
#[doc = "Field `PC9_UW` writer - Protection context 9, user write enable."]
pub type Pc9UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC9_PR` reader - Protection context 9, privileged read enable."]
pub type Pc9PrR = crate::BitReader;
#[doc = "Field `PC9_PR` writer - Protection context 9, privileged read enable."]
pub type Pc9PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC9_PW` reader - Protection context 9, privileged write enable."]
pub type Pc9PwR = crate::BitReader;
#[doc = "Field `PC9_PW` writer - Protection context 9, privileged write enable."]
pub type Pc9PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC9_NS` reader - Protection context 9, non-secure."]
pub type Pc9NsR = crate::BitReader;
#[doc = "Field `PC9_NS` writer - Protection context 9, non-secure."]
pub type Pc9NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC10_UR` reader - Protection context 10, user read enable."]
pub type Pc10UrR = crate::BitReader;
#[doc = "Field `PC10_UR` writer - Protection context 10, user read enable."]
pub type Pc10UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC10_UW` reader - Protection context 10, user write enable."]
pub type Pc10UwR = crate::BitReader;
#[doc = "Field `PC10_UW` writer - Protection context 10, user write enable."]
pub type Pc10UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC10_PR` reader - Protection context 10, privileged read enable."]
pub type Pc10PrR = crate::BitReader;
#[doc = "Field `PC10_PR` writer - Protection context 10, privileged read enable."]
pub type Pc10PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC10_PW` reader - Protection context 10, privileged write enable."]
pub type Pc10PwR = crate::BitReader;
#[doc = "Field `PC10_PW` writer - Protection context 10, privileged write enable."]
pub type Pc10PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC10_NS` reader - Protection context 10, non-secure."]
pub type Pc10NsR = crate::BitReader;
#[doc = "Field `PC10_NS` writer - Protection context 10, non-secure."]
pub type Pc10NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC11_UR` reader - Protection context 11, user read enable."]
pub type Pc11UrR = crate::BitReader;
#[doc = "Field `PC11_UR` writer - Protection context 11, user read enable."]
pub type Pc11UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC11_UW` reader - Protection context 11, user write enable."]
pub type Pc11UwR = crate::BitReader;
#[doc = "Field `PC11_UW` writer - Protection context 11, user write enable."]
pub type Pc11UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC11_PR` reader - Protection context 11, privileged read enable."]
pub type Pc11PrR = crate::BitReader;
#[doc = "Field `PC11_PR` writer - Protection context 11, privileged read enable."]
pub type Pc11PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC11_PW` reader - Protection context 11, privileged write enable."]
pub type Pc11PwR = crate::BitReader;
#[doc = "Field `PC11_PW` writer - Protection context 11, privileged write enable."]
pub type Pc11PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC11_NS` reader - Protection context 11, non-secure."]
pub type Pc11NsR = crate::BitReader;
#[doc = "Field `PC11_NS` writer - Protection context 11, non-secure."]
pub type Pc11NsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Protection context 8, user read enable."]
    #[inline(always)]
    pub fn pc8_ur(&self) -> Pc8UrR {
        Pc8UrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 8, user write enable."]
    #[inline(always)]
    pub fn pc8_uw(&self) -> Pc8UwR {
        Pc8UwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 8, privileged read enable."]
    #[inline(always)]
    pub fn pc8_pr(&self) -> Pc8PrR {
        Pc8PrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn pc8_pw(&self) -> Pc8PwR {
        Pc8PwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 8, non-secure."]
    #[inline(always)]
    pub fn pc8_ns(&self) -> Pc8NsR {
        Pc8NsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 9, user read enable."]
    #[inline(always)]
    pub fn pc9_ur(&self) -> Pc9UrR {
        Pc9UrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 9, user write enable."]
    #[inline(always)]
    pub fn pc9_uw(&self) -> Pc9UwR {
        Pc9UwR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 9, privileged read enable."]
    #[inline(always)]
    pub fn pc9_pr(&self) -> Pc9PrR {
        Pc9PrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn pc9_pw(&self) -> Pc9PwR {
        Pc9PwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 9, non-secure."]
    #[inline(always)]
    pub fn pc9_ns(&self) -> Pc9NsR {
        Pc9NsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 10, user read enable."]
    #[inline(always)]
    pub fn pc10_ur(&self) -> Pc10UrR {
        Pc10UrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 10, user write enable."]
    #[inline(always)]
    pub fn pc10_uw(&self) -> Pc10UwR {
        Pc10UwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 10, privileged read enable."]
    #[inline(always)]
    pub fn pc10_pr(&self) -> Pc10PrR {
        Pc10PrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn pc10_pw(&self) -> Pc10PwR {
        Pc10PwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 10, non-secure."]
    #[inline(always)]
    pub fn pc10_ns(&self) -> Pc10NsR {
        Pc10NsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 11, user read enable."]
    #[inline(always)]
    pub fn pc11_ur(&self) -> Pc11UrR {
        Pc11UrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 11, user write enable."]
    #[inline(always)]
    pub fn pc11_uw(&self) -> Pc11UwR {
        Pc11UwR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 11, privileged read enable."]
    #[inline(always)]
    pub fn pc11_pr(&self) -> Pc11PrR {
        Pc11PrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn pc11_pw(&self) -> Pc11PwR {
        Pc11PwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 11, non-secure."]
    #[inline(always)]
    pub fn pc11_ns(&self) -> Pc11NsR {
        Pc11NsR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protection context 8, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_ur(&mut self) -> Pc8UrW<SlAtt2Spec> {
        Pc8UrW::new(self, 0)
    }
    #[doc = "Bit 1 - Protection context 8, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_uw(&mut self) -> Pc8UwW<SlAtt2Spec> {
        Pc8UwW::new(self, 1)
    }
    #[doc = "Bit 2 - Protection context 8, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_pr(&mut self) -> Pc8PrW<SlAtt2Spec> {
        Pc8PrW::new(self, 2)
    }
    #[doc = "Bit 3 - Protection context 8, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_pw(&mut self) -> Pc8PwW<SlAtt2Spec> {
        Pc8PwW::new(self, 3)
    }
    #[doc = "Bit 4 - Protection context 8, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_ns(&mut self) -> Pc8NsW<SlAtt2Spec> {
        Pc8NsW::new(self, 4)
    }
    #[doc = "Bit 8 - Protection context 9, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_ur(&mut self) -> Pc9UrW<SlAtt2Spec> {
        Pc9UrW::new(self, 8)
    }
    #[doc = "Bit 9 - Protection context 9, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_uw(&mut self) -> Pc9UwW<SlAtt2Spec> {
        Pc9UwW::new(self, 9)
    }
    #[doc = "Bit 10 - Protection context 9, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_pr(&mut self) -> Pc9PrW<SlAtt2Spec> {
        Pc9PrW::new(self, 10)
    }
    #[doc = "Bit 11 - Protection context 9, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_pw(&mut self) -> Pc9PwW<SlAtt2Spec> {
        Pc9PwW::new(self, 11)
    }
    #[doc = "Bit 12 - Protection context 9, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_ns(&mut self) -> Pc9NsW<SlAtt2Spec> {
        Pc9NsW::new(self, 12)
    }
    #[doc = "Bit 16 - Protection context 10, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_ur(&mut self) -> Pc10UrW<SlAtt2Spec> {
        Pc10UrW::new(self, 16)
    }
    #[doc = "Bit 17 - Protection context 10, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_uw(&mut self) -> Pc10UwW<SlAtt2Spec> {
        Pc10UwW::new(self, 17)
    }
    #[doc = "Bit 18 - Protection context 10, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_pr(&mut self) -> Pc10PrW<SlAtt2Spec> {
        Pc10PrW::new(self, 18)
    }
    #[doc = "Bit 19 - Protection context 10, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_pw(&mut self) -> Pc10PwW<SlAtt2Spec> {
        Pc10PwW::new(self, 19)
    }
    #[doc = "Bit 20 - Protection context 10, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_ns(&mut self) -> Pc10NsW<SlAtt2Spec> {
        Pc10NsW::new(self, 20)
    }
    #[doc = "Bit 24 - Protection context 11, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_ur(&mut self) -> Pc11UrW<SlAtt2Spec> {
        Pc11UrW::new(self, 24)
    }
    #[doc = "Bit 25 - Protection context 11, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_uw(&mut self) -> Pc11UwW<SlAtt2Spec> {
        Pc11UwW::new(self, 25)
    }
    #[doc = "Bit 26 - Protection context 11, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_pr(&mut self) -> Pc11PrW<SlAtt2Spec> {
        Pc11PrW::new(self, 26)
    }
    #[doc = "Bit 27 - Protection context 11, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_pw(&mut self) -> Pc11PwW<SlAtt2Spec> {
        Pc11PwW::new(self, 27)
    }
    #[doc = "Bit 28 - Protection context 11, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_ns(&mut self) -> Pc11NsW<SlAtt2Spec> {
        Pc11NsW::new(self, 28)
    }
}
#[doc = "Slave attributes 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_att2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_att2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlAtt2Spec;
impl crate::RegisterSpec for SlAtt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl_att2::R`](R) reader structure"]
impl crate::Readable for SlAtt2Spec {}
#[doc = "`write(|w| ..)` method takes [`sl_att2::W`](W) writer structure"]
impl crate::Writable for SlAtt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL_ATT2 to value 0x1f1f_1f1f"]
impl crate::Resettable for SlAtt2Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
