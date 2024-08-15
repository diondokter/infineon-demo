#[doc = "Register `DESCR_Y_INCR` reader"]
pub type R = crate::R<DescrYIncrSpec>;
#[doc = "Field `SRC_Y` reader - Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
pub type SrcYR = crate::FieldReader<u16>;
#[doc = "Field `DST_Y` reader - Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
pub type DstYR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
    #[inline(always)]
    pub fn src_y(&self) -> SrcYR {
        SrcYR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
    #[inline(always)]
    pub fn dst_y(&self) -> DstYR {
        DstYR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor Y increment\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_y_incr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrYIncrSpec;
impl crate::RegisterSpec for DescrYIncrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_y_incr::R`](R) reader structure"]
impl crate::Readable for DescrYIncrSpec {}
#[doc = "`reset()` method sets DESCR_Y_INCR to value 0"]
impl crate::Resettable for DescrYIncrSpec {
    const RESET_VALUE: u32 = 0;
}
