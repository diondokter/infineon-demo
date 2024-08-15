#[doc = "Register `ERROR_STATUS0` reader"]
pub type R = crate::R<ErrorStatus0Spec>;
#[doc = "Field `DATA32` reader - Specifies error description information. - For INSTR_OPC_ERROR/ INSTR_CC_ERROR/ INSTR_DEV_KEY_ERROR: - Violating instruction (from instruction FIFO). - For BUS_ERROR: - Violating transfer, address."]
pub type Data32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies error description information. - For INSTR_OPC_ERROR/ INSTR_CC_ERROR/ INSTR_DEV_KEY_ERROR: - Violating instruction (from instruction FIFO). - For BUS_ERROR: - Violating transfer, address."]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
#[doc = "Error status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorStatus0Spec;
impl crate::RegisterSpec for ErrorStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_status0::R`](R) reader structure"]
impl crate::Readable for ErrorStatus0Spec {}
#[doc = "`reset()` method sets ERROR_STATUS0 to value 0"]
impl crate::Resettable for ErrorStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
