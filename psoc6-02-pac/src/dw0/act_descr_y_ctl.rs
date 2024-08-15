#[doc = "Register `ACT_DESCR_Y_CTL` reader"]
pub type R = crate::R<ActDescrYCtlSpec>;
#[doc = "Field `DATA` reader - Copy of DESCR_Y_CTL of the currently active descriptor. \\[11:0\\]
SRC_Y_INCR Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[23:12\\]
DST_Y_INCR Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[31:24\\]
Y_COUNT Number of iterations (minus 1) of the 'Y loop' (X_COUNT+1)*(Y_COUNT+1) is the number of single transfers in a 2D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For single, 1D and CRC transfer descriptor types, descriptor will not have Y_CTL."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_Y_CTL of the currently active descriptor. \\[11:0\\]
SRC_Y_INCR Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[23:12\\]
DST_Y_INCR Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[31:24\\]
Y_COUNT Number of iterations (minus 1) of the 'Y loop' (X_COUNT+1)*(Y_COUNT+1) is the number of single transfers in a 2D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For single, 1D and CRC transfer descriptor types, descriptor will not have Y_CTL."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Active descriptor Y loop control\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_y_ctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActDescrYCtlSpec;
impl crate::RegisterSpec for ActDescrYCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_y_ctl::R`](R) reader structure"]
impl crate::Readable for ActDescrYCtlSpec {}
#[doc = "`reset()` method sets ACT_DESCR_Y_CTL to value 0"]
impl crate::Resettable for ActDescrYCtlSpec {
    const RESET_VALUE: u32 = 0;
}
