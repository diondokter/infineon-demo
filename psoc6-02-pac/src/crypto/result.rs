#[doc = "Register `RESULT` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Register `RESULT` writer"]
pub type W = crate::W<ResultSpec>;
#[doc = "Field `DATA` reader - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<ResultSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`write(|w| ..)` method takes [`result::W`](W) writer structure"]
impl crate::Writable for ResultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for ResultSpec {
    const RESET_VALUE: u32 = 0;
}
