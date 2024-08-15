#[doc = "Register `DESCR_X_INCR` reader"]
pub type R = crate::R<DescrXIncrSpec>;
#[doc = "Field `SRC_X` reader - Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures."]
pub type SrcXR = crate::FieldReader<u16>;
#[doc = "Field `DST_X` reader - Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures."]
pub type DstXR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures."]
    #[inline(always)]
    pub fn src_x(&self) -> SrcXR {
        SrcXR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures."]
    #[inline(always)]
    pub fn dst_x(&self) -> DstXR {
        DstXR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor X increment\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_x_incr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrXIncrSpec;
impl crate::RegisterSpec for DescrXIncrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_x_incr::R`](R) reader structure"]
impl crate::Readable for DescrXIncrSpec {}
#[doc = "`reset()` method sets DESCR_X_INCR to value 0"]
impl crate::Resettable for DescrXIncrSpec {
    const RESET_VALUE: u32 = 0;
}
