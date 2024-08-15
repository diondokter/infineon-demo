#[doc = "Register `MS_ATT0` reader"]
pub type R = crate::R<MsAtt0Spec>;
#[doc = "Register `MS_ATT0` writer"]
pub type W = crate::W<MsAtt0Spec>;
#[doc = "Field `PC0_UR` reader - Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
pub type Pc0UrR = crate::BitReader;
#[doc = "Field `PC0_UW` reader - Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type Pc0UwR = crate::BitReader;
#[doc = "Field `PC0_PR` reader - Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
pub type Pc0PrR = crate::BitReader;
#[doc = "Field `PC0_PW` reader - Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type Pc0PwR = crate::BitReader;
#[doc = "Field `PC0_NS` reader - Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type Pc0NsR = crate::BitReader;
#[doc = "Field `PC1_UR` reader - Protection context 1, user read enable."]
pub type Pc1UrR = crate::BitReader;
#[doc = "Field `PC1_UW` reader - Protection context 1, user write enable."]
pub type Pc1UwR = crate::BitReader;
#[doc = "Field `PC1_UW` writer - Protection context 1, user write enable."]
pub type Pc1UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC1_PR` reader - Protection context 1, privileged read enable."]
pub type Pc1PrR = crate::BitReader;
#[doc = "Field `PC1_PW` reader - Protection context 1, privileged write enable."]
pub type Pc1PwR = crate::BitReader;
#[doc = "Field `PC1_PW` writer - Protection context 1, privileged write enable."]
pub type Pc1PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC1_NS` reader - Protection context 1, non-secure."]
pub type Pc1NsR = crate::BitReader;
#[doc = "Field `PC1_NS` writer - Protection context 1, non-secure."]
pub type Pc1NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2_UR` reader - Protection context 2, user read enable."]
pub type Pc2UrR = crate::BitReader;
#[doc = "Field `PC2_UW` reader - Protection context 2, user write enable."]
pub type Pc2UwR = crate::BitReader;
#[doc = "Field `PC2_UW` writer - Protection context 2, user write enable."]
pub type Pc2UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2_PR` reader - Protection context 2, privileged read enable."]
pub type Pc2PrR = crate::BitReader;
#[doc = "Field `PC2_PW` reader - Protection context 2, privileged write enable."]
pub type Pc2PwR = crate::BitReader;
#[doc = "Field `PC2_PW` writer - Protection context 2, privileged write enable."]
pub type Pc2PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2_NS` reader - Protection context 2, non-secure."]
pub type Pc2NsR = crate::BitReader;
#[doc = "Field `PC2_NS` writer - Protection context 2, non-secure."]
pub type Pc2NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3_UR` reader - Protection context 3, user read enable."]
pub type Pc3UrR = crate::BitReader;
#[doc = "Field `PC3_UW` reader - Protection context 3, user write enable."]
pub type Pc3UwR = crate::BitReader;
#[doc = "Field `PC3_UW` writer - Protection context 3, user write enable."]
pub type Pc3UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3_PR` reader - Protection context 3, privileged read enable."]
pub type Pc3PrR = crate::BitReader;
#[doc = "Field `PC3_PW` reader - Protection context 3, privileged write enable."]
pub type Pc3PwR = crate::BitReader;
#[doc = "Field `PC3_PW` writer - Protection context 3, privileged write enable."]
pub type Pc3PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3_NS` reader - Protection context 3, non-secure."]
pub type Pc3NsR = crate::BitReader;
#[doc = "Field `PC3_NS` writer - Protection context 3, non-secure."]
pub type Pc3NsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_ur(&self) -> Pc0UrR {
        Pc0UrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_uw(&self) -> Pc0UwR {
        Pc0UwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_pr(&self) -> Pc0PrR {
        Pc0PrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_pw(&self) -> Pc0PwR {
        Pc0PwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn pc0_ns(&self) -> Pc0NsR {
        Pc0NsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 1, user read enable."]
    #[inline(always)]
    pub fn pc1_ur(&self) -> Pc1UrR {
        Pc1UrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 1, user write enable."]
    #[inline(always)]
    pub fn pc1_uw(&self) -> Pc1UwR {
        Pc1UwR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 1, privileged read enable."]
    #[inline(always)]
    pub fn pc1_pr(&self) -> Pc1PrR {
        Pc1PrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn pc1_pw(&self) -> Pc1PwR {
        Pc1PwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 1, non-secure."]
    #[inline(always)]
    pub fn pc1_ns(&self) -> Pc1NsR {
        Pc1NsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 2, user read enable."]
    #[inline(always)]
    pub fn pc2_ur(&self) -> Pc2UrR {
        Pc2UrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 2, user write enable."]
    #[inline(always)]
    pub fn pc2_uw(&self) -> Pc2UwR {
        Pc2UwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 2, privileged read enable."]
    #[inline(always)]
    pub fn pc2_pr(&self) -> Pc2PrR {
        Pc2PrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn pc2_pw(&self) -> Pc2PwR {
        Pc2PwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 2, non-secure."]
    #[inline(always)]
    pub fn pc2_ns(&self) -> Pc2NsR {
        Pc2NsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 3, user read enable."]
    #[inline(always)]
    pub fn pc3_ur(&self) -> Pc3UrR {
        Pc3UrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 3, user write enable."]
    #[inline(always)]
    pub fn pc3_uw(&self) -> Pc3UwR {
        Pc3UwR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 3, privileged read enable."]
    #[inline(always)]
    pub fn pc3_pr(&self) -> Pc3PrR {
        Pc3PrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn pc3_pw(&self) -> Pc3PwR {
        Pc3PwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 3, non-secure."]
    #[inline(always)]
    pub fn pc3_ns(&self) -> Pc3NsR {
        Pc3NsR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Protection context 1, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_uw(&mut self) -> Pc1UwW<MsAtt0Spec> {
        Pc1UwW::new(self, 9)
    }
    #[doc = "Bit 11 - Protection context 1, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_pw(&mut self) -> Pc1PwW<MsAtt0Spec> {
        Pc1PwW::new(self, 11)
    }
    #[doc = "Bit 12 - Protection context 1, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_ns(&mut self) -> Pc1NsW<MsAtt0Spec> {
        Pc1NsW::new(self, 12)
    }
    #[doc = "Bit 17 - Protection context 2, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_uw(&mut self) -> Pc2UwW<MsAtt0Spec> {
        Pc2UwW::new(self, 17)
    }
    #[doc = "Bit 19 - Protection context 2, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_pw(&mut self) -> Pc2PwW<MsAtt0Spec> {
        Pc2PwW::new(self, 19)
    }
    #[doc = "Bit 20 - Protection context 2, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_ns(&mut self) -> Pc2NsW<MsAtt0Spec> {
        Pc2NsW::new(self, 20)
    }
    #[doc = "Bit 25 - Protection context 3, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_uw(&mut self) -> Pc3UwW<MsAtt0Spec> {
        Pc3UwW::new(self, 25)
    }
    #[doc = "Bit 27 - Protection context 3, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_pw(&mut self) -> Pc3PwW<MsAtt0Spec> {
        Pc3PwW::new(self, 27)
    }
    #[doc = "Bit 28 - Protection context 3, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_ns(&mut self) -> Pc3NsW<MsAtt0Spec> {
        Pc3NsW::new(self, 28)
    }
}
#[doc = "Master attributes 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_att0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_att0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsAtt0Spec;
impl crate::RegisterSpec for MsAtt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_att0::R`](R) reader structure"]
impl crate::Readable for MsAtt0Spec {}
#[doc = "`write(|w| ..)` method takes [`ms_att0::W`](W) writer structure"]
impl crate::Writable for MsAtt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MS_ATT0 to value 0x1f1f_1f1f"]
impl crate::Resettable for MsAtt0Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
