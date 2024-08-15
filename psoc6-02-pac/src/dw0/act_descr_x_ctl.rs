#[doc = "Register `ACT_DESCR_X_CTL` reader"]
pub type R = crate::R<ActDescrXCtlSpec>;
#[doc = "Field `DATA` reader - Copy of DESCR_X_CTL of the currently active descriptor. \\[11:0\\]
SRC_X_INCR Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures. \\[23:12\\]
DST_X_INCR Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures. Note: this field is not used for CRC transfer descriptors and must be set to '0'. \\[31:24\\]
X_COUNT Number of iterations (minus 1) of the 'X loop' (X_COUNT+1 is the number of single transfers in a 1D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For a single transfer descriptor type, descriptor will not have X_CTL."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_X_CTL of the currently active descriptor. \\[11:0\\]
SRC_X_INCR Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures. \\[23:12\\]
DST_X_INCR Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures. Note: this field is not used for CRC transfer descriptors and must be set to '0'. \\[31:24\\]
X_COUNT Number of iterations (minus 1) of the 'X loop' (X_COUNT+1 is the number of single transfers in a 1D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For a single transfer descriptor type, descriptor will not have X_CTL."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Active descriptor X loop control\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_x_ctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActDescrXCtlSpec;
impl crate::RegisterSpec for ActDescrXCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_x_ctl::R`](R) reader structure"]
impl crate::Readable for ActDescrXCtlSpec {}
#[doc = "`reset()` method sets ACT_DESCR_X_CTL to value 0"]
impl crate::Resettable for ActDescrXCtlSpec {
    const RESET_VALUE: u32 = 0;
}
