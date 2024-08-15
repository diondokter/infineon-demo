#[doc = "Register `DESCR_Y_SIZE` reader"]
pub type R = crate::R<DescrYSizeSpec>;
#[doc = "Field `Y_COUNT` reader - Number of iterations (minus 1) of the 'Y loop' (X_COUNT+1)*(Y_COUNT+1) is the number of single transfers in a 2D transfer). This field is an unsigned number in the range \\[0, 65535\\], representing 1 through 65536 iterations."]
pub type YCountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of iterations (minus 1) of the 'Y loop' (X_COUNT+1)*(Y_COUNT+1) is the number of single transfers in a 2D transfer). This field is an unsigned number in the range \\[0, 65535\\], representing 1 through 65536 iterations."]
    #[inline(always)]
    pub fn y_count(&self) -> YCountR {
        YCountR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor Y size\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_y_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrYSizeSpec;
impl crate::RegisterSpec for DescrYSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_y_size::R`](R) reader structure"]
impl crate::Readable for DescrYSizeSpec {}
#[doc = "`reset()` method sets DESCR_Y_SIZE to value 0"]
impl crate::Resettable for DescrYSizeSpec {
    const RESET_VALUE: u32 = 0;
}
