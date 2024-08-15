#[doc = "Register `SL_ATT1` reader"]
pub type R = crate::R<SlAtt1Spec>;
#[doc = "Register `SL_ATT1` writer"]
pub type W = crate::W<SlAtt1Spec>;
#[doc = "Field `PC4_UR` reader - Protection context 4, user read enable."]
pub type Pc4UrR = crate::BitReader;
#[doc = "Field `PC4_UR` writer - Protection context 4, user read enable."]
pub type Pc4UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_UW` reader - Protection context 4, user write enable."]
pub type Pc4UwR = crate::BitReader;
#[doc = "Field `PC4_UW` writer - Protection context 4, user write enable."]
pub type Pc4UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_PR` reader - Protection context 4, privileged read enable."]
pub type Pc4PrR = crate::BitReader;
#[doc = "Field `PC4_PR` writer - Protection context 4, privileged read enable."]
pub type Pc4PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_PW` reader - Protection context 4, privileged write enable."]
pub type Pc4PwR = crate::BitReader;
#[doc = "Field `PC4_PW` writer - Protection context 4, privileged write enable."]
pub type Pc4PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_NS` reader - Protection context 4, non-secure."]
pub type Pc4NsR = crate::BitReader;
#[doc = "Field `PC4_NS` writer - Protection context 4, non-secure."]
pub type Pc4NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_UR` reader - Protection context 5, user read enable."]
pub type Pc5UrR = crate::BitReader;
#[doc = "Field `PC5_UR` writer - Protection context 5, user read enable."]
pub type Pc5UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_UW` reader - Protection context 5, user write enable."]
pub type Pc5UwR = crate::BitReader;
#[doc = "Field `PC5_UW` writer - Protection context 5, user write enable."]
pub type Pc5UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_PR` reader - Protection context 5, privileged read enable."]
pub type Pc5PrR = crate::BitReader;
#[doc = "Field `PC5_PR` writer - Protection context 5, privileged read enable."]
pub type Pc5PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_PW` reader - Protection context 5, privileged write enable."]
pub type Pc5PwR = crate::BitReader;
#[doc = "Field `PC5_PW` writer - Protection context 5, privileged write enable."]
pub type Pc5PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_NS` reader - Protection context 5, non-secure."]
pub type Pc5NsR = crate::BitReader;
#[doc = "Field `PC5_NS` writer - Protection context 5, non-secure."]
pub type Pc5NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_UR` reader - Protection context 6, user read enable."]
pub type Pc6UrR = crate::BitReader;
#[doc = "Field `PC6_UR` writer - Protection context 6, user read enable."]
pub type Pc6UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_UW` reader - Protection context 6, user write enable."]
pub type Pc6UwR = crate::BitReader;
#[doc = "Field `PC6_UW` writer - Protection context 6, user write enable."]
pub type Pc6UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_PR` reader - Protection context 6, privileged read enable."]
pub type Pc6PrR = crate::BitReader;
#[doc = "Field `PC6_PR` writer - Protection context 6, privileged read enable."]
pub type Pc6PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_PW` reader - Protection context 6, privileged write enable."]
pub type Pc6PwR = crate::BitReader;
#[doc = "Field `PC6_PW` writer - Protection context 6, privileged write enable."]
pub type Pc6PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_NS` reader - Protection context 6, non-secure."]
pub type Pc6NsR = crate::BitReader;
#[doc = "Field `PC6_NS` writer - Protection context 6, non-secure."]
pub type Pc6NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_UR` reader - Protection context 7, user read enable."]
pub type Pc7UrR = crate::BitReader;
#[doc = "Field `PC7_UR` writer - Protection context 7, user read enable."]
pub type Pc7UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_UW` reader - Protection context 7, user write enable."]
pub type Pc7UwR = crate::BitReader;
#[doc = "Field `PC7_UW` writer - Protection context 7, user write enable."]
pub type Pc7UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_PR` reader - Protection context 7, privileged read enable."]
pub type Pc7PrR = crate::BitReader;
#[doc = "Field `PC7_PR` writer - Protection context 7, privileged read enable."]
pub type Pc7PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_PW` reader - Protection context 7, privileged write enable."]
pub type Pc7PwR = crate::BitReader;
#[doc = "Field `PC7_PW` writer - Protection context 7, privileged write enable."]
pub type Pc7PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_NS` reader - Protection context 7, non-secure."]
pub type Pc7NsR = crate::BitReader;
#[doc = "Field `PC7_NS` writer - Protection context 7, non-secure."]
pub type Pc7NsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Protection context 4, user read enable."]
    #[inline(always)]
    pub fn pc4_ur(&self) -> Pc4UrR {
        Pc4UrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 4, user write enable."]
    #[inline(always)]
    pub fn pc4_uw(&self) -> Pc4UwR {
        Pc4UwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 4, privileged read enable."]
    #[inline(always)]
    pub fn pc4_pr(&self) -> Pc4PrR {
        Pc4PrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn pc4_pw(&self) -> Pc4PwR {
        Pc4PwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 4, non-secure."]
    #[inline(always)]
    pub fn pc4_ns(&self) -> Pc4NsR {
        Pc4NsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 5, user read enable."]
    #[inline(always)]
    pub fn pc5_ur(&self) -> Pc5UrR {
        Pc5UrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 5, user write enable."]
    #[inline(always)]
    pub fn pc5_uw(&self) -> Pc5UwR {
        Pc5UwR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 5, privileged read enable."]
    #[inline(always)]
    pub fn pc5_pr(&self) -> Pc5PrR {
        Pc5PrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn pc5_pw(&self) -> Pc5PwR {
        Pc5PwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 5, non-secure."]
    #[inline(always)]
    pub fn pc5_ns(&self) -> Pc5NsR {
        Pc5NsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 6, user read enable."]
    #[inline(always)]
    pub fn pc6_ur(&self) -> Pc6UrR {
        Pc6UrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 6, user write enable."]
    #[inline(always)]
    pub fn pc6_uw(&self) -> Pc6UwR {
        Pc6UwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 6, privileged read enable."]
    #[inline(always)]
    pub fn pc6_pr(&self) -> Pc6PrR {
        Pc6PrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn pc6_pw(&self) -> Pc6PwR {
        Pc6PwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 6, non-secure."]
    #[inline(always)]
    pub fn pc6_ns(&self) -> Pc6NsR {
        Pc6NsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 7, user read enable."]
    #[inline(always)]
    pub fn pc7_ur(&self) -> Pc7UrR {
        Pc7UrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 7, user write enable."]
    #[inline(always)]
    pub fn pc7_uw(&self) -> Pc7UwR {
        Pc7UwR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 7, privileged read enable."]
    #[inline(always)]
    pub fn pc7_pr(&self) -> Pc7PrR {
        Pc7PrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn pc7_pw(&self) -> Pc7PwR {
        Pc7PwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 7, non-secure."]
    #[inline(always)]
    pub fn pc7_ns(&self) -> Pc7NsR {
        Pc7NsR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protection context 4, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_ur(&mut self) -> Pc4UrW<SlAtt1Spec> {
        Pc4UrW::new(self, 0)
    }
    #[doc = "Bit 1 - Protection context 4, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_uw(&mut self) -> Pc4UwW<SlAtt1Spec> {
        Pc4UwW::new(self, 1)
    }
    #[doc = "Bit 2 - Protection context 4, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_pr(&mut self) -> Pc4PrW<SlAtt1Spec> {
        Pc4PrW::new(self, 2)
    }
    #[doc = "Bit 3 - Protection context 4, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_pw(&mut self) -> Pc4PwW<SlAtt1Spec> {
        Pc4PwW::new(self, 3)
    }
    #[doc = "Bit 4 - Protection context 4, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_ns(&mut self) -> Pc4NsW<SlAtt1Spec> {
        Pc4NsW::new(self, 4)
    }
    #[doc = "Bit 8 - Protection context 5, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_ur(&mut self) -> Pc5UrW<SlAtt1Spec> {
        Pc5UrW::new(self, 8)
    }
    #[doc = "Bit 9 - Protection context 5, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_uw(&mut self) -> Pc5UwW<SlAtt1Spec> {
        Pc5UwW::new(self, 9)
    }
    #[doc = "Bit 10 - Protection context 5, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_pr(&mut self) -> Pc5PrW<SlAtt1Spec> {
        Pc5PrW::new(self, 10)
    }
    #[doc = "Bit 11 - Protection context 5, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_pw(&mut self) -> Pc5PwW<SlAtt1Spec> {
        Pc5PwW::new(self, 11)
    }
    #[doc = "Bit 12 - Protection context 5, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_ns(&mut self) -> Pc5NsW<SlAtt1Spec> {
        Pc5NsW::new(self, 12)
    }
    #[doc = "Bit 16 - Protection context 6, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_ur(&mut self) -> Pc6UrW<SlAtt1Spec> {
        Pc6UrW::new(self, 16)
    }
    #[doc = "Bit 17 - Protection context 6, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_uw(&mut self) -> Pc6UwW<SlAtt1Spec> {
        Pc6UwW::new(self, 17)
    }
    #[doc = "Bit 18 - Protection context 6, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_pr(&mut self) -> Pc6PrW<SlAtt1Spec> {
        Pc6PrW::new(self, 18)
    }
    #[doc = "Bit 19 - Protection context 6, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_pw(&mut self) -> Pc6PwW<SlAtt1Spec> {
        Pc6PwW::new(self, 19)
    }
    #[doc = "Bit 20 - Protection context 6, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_ns(&mut self) -> Pc6NsW<SlAtt1Spec> {
        Pc6NsW::new(self, 20)
    }
    #[doc = "Bit 24 - Protection context 7, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_ur(&mut self) -> Pc7UrW<SlAtt1Spec> {
        Pc7UrW::new(self, 24)
    }
    #[doc = "Bit 25 - Protection context 7, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_uw(&mut self) -> Pc7UwW<SlAtt1Spec> {
        Pc7UwW::new(self, 25)
    }
    #[doc = "Bit 26 - Protection context 7, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_pr(&mut self) -> Pc7PrW<SlAtt1Spec> {
        Pc7PrW::new(self, 26)
    }
    #[doc = "Bit 27 - Protection context 7, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_pw(&mut self) -> Pc7PwW<SlAtt1Spec> {
        Pc7PwW::new(self, 27)
    }
    #[doc = "Bit 28 - Protection context 7, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_ns(&mut self) -> Pc7NsW<SlAtt1Spec> {
        Pc7NsW::new(self, 28)
    }
}
#[doc = "Slave attributes 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_att1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_att1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlAtt1Spec;
impl crate::RegisterSpec for SlAtt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl_att1::R`](R) reader structure"]
impl crate::Readable for SlAtt1Spec {}
#[doc = "`write(|w| ..)` method takes [`sl_att1::W`](W) writer structure"]
impl crate::Writable for SlAtt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL_ATT1 to value 0x1f1f_1f1f"]
impl crate::Resettable for SlAtt1Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
