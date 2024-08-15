#[doc = "Register `DESCR_X_SIZE` reader"]
pub type R = crate::R<DescrXSizeSpec>;
#[doc = "Field `X_COUNT` reader - Number of iterations (minus 1) of the 'X loop' (X_COUNT+1 is the number of single transfers in a 1D transfer). This field is an unsigned number in the range \\[0, 65535\\], representing 1 through 65536 iterations. For the 'memory copy' descriptor type, (X_COUNT + 1) is the number of transferred Bytes. For the 'scatter' descriptor type, ceiling(X_COUNT/2) is the number of (address, write data) initialization pairs processed."]
pub type XCountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of iterations (minus 1) of the 'X loop' (X_COUNT+1 is the number of single transfers in a 1D transfer). This field is an unsigned number in the range \\[0, 65535\\], representing 1 through 65536 iterations. For the 'memory copy' descriptor type, (X_COUNT + 1) is the number of transferred Bytes. For the 'scatter' descriptor type, ceiling(X_COUNT/2) is the number of (address, write data) initialization pairs processed."]
    #[inline(always)]
    pub fn x_count(&self) -> XCountR {
        XCountR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor X size\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_x_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrXSizeSpec;
impl crate::RegisterSpec for DescrXSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_x_size::R`](R) reader structure"]
impl crate::Readable for DescrXSizeSpec {}
#[doc = "`reset()` method sets DESCR_X_SIZE to value 0"]
impl crate::Resettable for DescrXSizeSpec {
    const RESET_VALUE: u32 = 0;
}
