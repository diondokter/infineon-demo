#[doc = "Register `DATA[%s]` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA[%s]` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA` reader - Captured fault source data. Note: the DATA registers can only be written when STATUS.VALID is '0'. Note: the fault source index STATUS.IDX specifies the format of the DATA registers."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Captured fault source data. Note: the DATA registers can only be written when STATUS.VALID is '0'. Note: the fault source index STATUS.IDX specifies the format of the DATA registers."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Captured fault source data. Note: the DATA registers can only be written when STATUS.VALID is '0'. Note: the fault source index STATUS.IDX specifies the format of the DATA registers."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Captured fault source data. Note: the DATA registers can only be written when STATUS.VALID is '0'. Note: the fault source index STATUS.IDX specifies the format of the DATA registers."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Fault data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA[%s]
to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
