#[doc = "Register `BREG[%s]` reader"]
pub type R = crate::R<BregSpec>;
#[doc = "Register `BREG[%s]` writer"]
pub type W = crate::W<BregSpec>;
#[doc = "Field `BREG` reader - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
pub type BregR = crate::FieldReader<u32>;
#[doc = "Field `BREG` writer - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
pub type BregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    pub fn breg(&self) -> BregR {
        BregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    #[must_use]
    pub fn breg(&mut self) -> BregW<BregSpec> {
        BregW::new(self, 0)
    }
}
#[doc = "Backup register region\n\nYou can [`read`](crate::Reg::read) this register and get [`breg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`breg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BregSpec;
impl crate::RegisterSpec for BregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`breg::R`](R) reader structure"]
impl crate::Readable for BregSpec {}
#[doc = "`write(|w| ..)` method takes [`breg::W`](W) writer structure"]
impl crate::Writable for BregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BREG[%s]
to value 0"]
impl crate::Resettable for BregSpec {
    const RESET_VALUE: u32 = 0;
}
