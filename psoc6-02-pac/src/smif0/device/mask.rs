#[doc = "Register `MASK` reader"]
pub type R = crate::R<MaskSpec>;
#[doc = "Register `MASK` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `MASK` reader - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<MaskSpec> {
        MaskW::new(self, 8)
    }
}
#[doc = "Device region mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSpec;
impl crate::RegisterSpec for MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MaskSpec {
    const RESET_VALUE: u32 = 0;
}
