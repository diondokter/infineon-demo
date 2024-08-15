#[doc = "Register `MS_ATT3` reader"]
pub type R = crate::R<MsAtt3Spec>;
#[doc = "Register `MS_ATT3` writer"]
pub type W = crate::W<MsAtt3Spec>;
#[doc = "Field `PC12_UR` reader - Protection context 12, user read enable."]
pub type Pc12UrR = crate::BitReader;
#[doc = "Field `PC12_UW` reader - Protection context 12, user write enable."]
pub type Pc12UwR = crate::BitReader;
#[doc = "Field `PC12_UW` writer - Protection context 12, user write enable."]
pub type Pc12UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC12_PR` reader - Protection context 12, privileged read enable."]
pub type Pc12PrR = crate::BitReader;
#[doc = "Field `PC12_PW` reader - Protection context 12, privileged write enable."]
pub type Pc12PwR = crate::BitReader;
#[doc = "Field `PC12_PW` writer - Protection context 12, privileged write enable."]
pub type Pc12PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC12_NS` reader - Protection context 12, non-secure."]
pub type Pc12NsR = crate::BitReader;
#[doc = "Field `PC12_NS` writer - Protection context 12, non-secure."]
pub type Pc12NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC13_UR` reader - Protection context 13, user read enable."]
pub type Pc13UrR = crate::BitReader;
#[doc = "Field `PC13_UW` reader - Protection context 13, user write enable."]
pub type Pc13UwR = crate::BitReader;
#[doc = "Field `PC13_UW` writer - Protection context 13, user write enable."]
pub type Pc13UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC13_PR` reader - Protection context 13, privileged read enable."]
pub type Pc13PrR = crate::BitReader;
#[doc = "Field `PC13_PW` reader - Protection context 13, privileged write enable."]
pub type Pc13PwR = crate::BitReader;
#[doc = "Field `PC13_PW` writer - Protection context 13, privileged write enable."]
pub type Pc13PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC13_NS` reader - Protection context 13, non-secure."]
pub type Pc13NsR = crate::BitReader;
#[doc = "Field `PC13_NS` writer - Protection context 13, non-secure."]
pub type Pc13NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14_UR` reader - Protection context 14, user read enable."]
pub type Pc14UrR = crate::BitReader;
#[doc = "Field `PC14_UW` reader - Protection context 14, user write enable."]
pub type Pc14UwR = crate::BitReader;
#[doc = "Field `PC14_UW` writer - Protection context 14, user write enable."]
pub type Pc14UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14_PR` reader - Protection context 14, privileged read enable."]
pub type Pc14PrR = crate::BitReader;
#[doc = "Field `PC14_PW` reader - Protection context 14, privileged write enable."]
pub type Pc14PwR = crate::BitReader;
#[doc = "Field `PC14_PW` writer - Protection context 14, privileged write enable."]
pub type Pc14PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14_NS` reader - Protection context 14, non-secure."]
pub type Pc14NsR = crate::BitReader;
#[doc = "Field `PC14_NS` writer - Protection context 14, non-secure."]
pub type Pc14NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15_UR` reader - Protection context 15, user read enable."]
pub type Pc15UrR = crate::BitReader;
#[doc = "Field `PC15_UW` reader - Protection context 15, user write enable."]
pub type Pc15UwR = crate::BitReader;
#[doc = "Field `PC15_UW` writer - Protection context 15, user write enable."]
pub type Pc15UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15_PR` reader - Protection context 15, privileged read enable."]
pub type Pc15PrR = crate::BitReader;
#[doc = "Field `PC15_PW` reader - Protection context 15, privileged write enable."]
pub type Pc15PwR = crate::BitReader;
#[doc = "Field `PC15_PW` writer - Protection context 15, privileged write enable."]
pub type Pc15PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15_NS` reader - Protection context 15, non-secure."]
pub type Pc15NsR = crate::BitReader;
#[doc = "Field `PC15_NS` writer - Protection context 15, non-secure."]
pub type Pc15NsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Protection context 12, user read enable."]
    #[inline(always)]
    pub fn pc12_ur(&self) -> Pc12UrR {
        Pc12UrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 12, user write enable."]
    #[inline(always)]
    pub fn pc12_uw(&self) -> Pc12UwR {
        Pc12UwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 12, privileged read enable."]
    #[inline(always)]
    pub fn pc12_pr(&self) -> Pc12PrR {
        Pc12PrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 12, privileged write enable."]
    #[inline(always)]
    pub fn pc12_pw(&self) -> Pc12PwR {
        Pc12PwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 12, non-secure."]
    #[inline(always)]
    pub fn pc12_ns(&self) -> Pc12NsR {
        Pc12NsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 13, user read enable."]
    #[inline(always)]
    pub fn pc13_ur(&self) -> Pc13UrR {
        Pc13UrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 13, user write enable."]
    #[inline(always)]
    pub fn pc13_uw(&self) -> Pc13UwR {
        Pc13UwR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 13, privileged read enable."]
    #[inline(always)]
    pub fn pc13_pr(&self) -> Pc13PrR {
        Pc13PrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 13, privileged write enable."]
    #[inline(always)]
    pub fn pc13_pw(&self) -> Pc13PwR {
        Pc13PwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 13, non-secure."]
    #[inline(always)]
    pub fn pc13_ns(&self) -> Pc13NsR {
        Pc13NsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 14, user read enable."]
    #[inline(always)]
    pub fn pc14_ur(&self) -> Pc14UrR {
        Pc14UrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 14, user write enable."]
    #[inline(always)]
    pub fn pc14_uw(&self) -> Pc14UwR {
        Pc14UwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 14, privileged read enable."]
    #[inline(always)]
    pub fn pc14_pr(&self) -> Pc14PrR {
        Pc14PrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 14, privileged write enable."]
    #[inline(always)]
    pub fn pc14_pw(&self) -> Pc14PwR {
        Pc14PwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 14, non-secure."]
    #[inline(always)]
    pub fn pc14_ns(&self) -> Pc14NsR {
        Pc14NsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 15, user read enable."]
    #[inline(always)]
    pub fn pc15_ur(&self) -> Pc15UrR {
        Pc15UrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 15, user write enable."]
    #[inline(always)]
    pub fn pc15_uw(&self) -> Pc15UwR {
        Pc15UwR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 15, privileged read enable."]
    #[inline(always)]
    pub fn pc15_pr(&self) -> Pc15PrR {
        Pc15PrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 15, privileged write enable."]
    #[inline(always)]
    pub fn pc15_pw(&self) -> Pc15PwR {
        Pc15PwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 15, non-secure."]
    #[inline(always)]
    pub fn pc15_ns(&self) -> Pc15NsR {
        Pc15NsR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Protection context 12, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc12_uw(&mut self) -> Pc12UwW<MsAtt3Spec> {
        Pc12UwW::new(self, 1)
    }
    #[doc = "Bit 3 - Protection context 12, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc12_pw(&mut self) -> Pc12PwW<MsAtt3Spec> {
        Pc12PwW::new(self, 3)
    }
    #[doc = "Bit 4 - Protection context 12, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc12_ns(&mut self) -> Pc12NsW<MsAtt3Spec> {
        Pc12NsW::new(self, 4)
    }
    #[doc = "Bit 9 - Protection context 13, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc13_uw(&mut self) -> Pc13UwW<MsAtt3Spec> {
        Pc13UwW::new(self, 9)
    }
    #[doc = "Bit 11 - Protection context 13, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc13_pw(&mut self) -> Pc13PwW<MsAtt3Spec> {
        Pc13PwW::new(self, 11)
    }
    #[doc = "Bit 12 - Protection context 13, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc13_ns(&mut self) -> Pc13NsW<MsAtt3Spec> {
        Pc13NsW::new(self, 12)
    }
    #[doc = "Bit 17 - Protection context 14, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc14_uw(&mut self) -> Pc14UwW<MsAtt3Spec> {
        Pc14UwW::new(self, 17)
    }
    #[doc = "Bit 19 - Protection context 14, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc14_pw(&mut self) -> Pc14PwW<MsAtt3Spec> {
        Pc14PwW::new(self, 19)
    }
    #[doc = "Bit 20 - Protection context 14, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc14_ns(&mut self) -> Pc14NsW<MsAtt3Spec> {
        Pc14NsW::new(self, 20)
    }
    #[doc = "Bit 25 - Protection context 15, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc15_uw(&mut self) -> Pc15UwW<MsAtt3Spec> {
        Pc15UwW::new(self, 25)
    }
    #[doc = "Bit 27 - Protection context 15, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc15_pw(&mut self) -> Pc15PwW<MsAtt3Spec> {
        Pc15PwW::new(self, 27)
    }
    #[doc = "Bit 28 - Protection context 15, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc15_ns(&mut self) -> Pc15NsW<MsAtt3Spec> {
        Pc15NsW::new(self, 28)
    }
}
#[doc = "Master attributes 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_att3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_att3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsAtt3Spec;
impl crate::RegisterSpec for MsAtt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_att3::R`](R) reader structure"]
impl crate::Readable for MsAtt3Spec {}
#[doc = "`write(|w| ..)` method takes [`ms_att3::W`](W) writer structure"]
impl crate::Writable for MsAtt3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MS_ATT3 to value 0x1f1f_1f1f"]
impl crate::Resettable for MsAtt3Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
