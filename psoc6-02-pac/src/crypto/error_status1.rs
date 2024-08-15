#[doc = "Register `ERROR_STATUS1` reader"]
pub type R = crate::R<ErrorStatus1Spec>;
#[doc = "Register `ERROR_STATUS1` writer"]
pub type W = crate::W<ErrorStatus1Spec>;
#[doc = "Field `DATA24` reader - Specifies error description information. - For BUS_ERROR: - Violating transfer, read attribute (DATA\\[0\\]). - Violating transfer, size attribute (DATA\\[5:4\\]). '0': 8-bit transfer, '1': 16 bits transfer, '2': 32-bit transfer."]
pub type Data24R = crate::FieldReader<u32>;
#[doc = "Field `IDX` reader - Error source: '0': INSTR_OPC_ERROR (instruction FIFO decoder error). '1': INSTR_CC_ERROR (instruction FIFO decoder, VU CC error). '2': BUS_ERROR (bus master interface AHB-Lite bus error). '3': TR_AP_DETECT_ERROR. '4': TR_RC_DETECT_ERROR. '5': INSTR_DEV_KEY_ERROR. '6'-'7': Undefined."]
pub type IdxR = crate::FieldReader;
#[doc = "Field `VALID` reader - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Specifies error description information. - For BUS_ERROR: - Violating transfer, read attribute (DATA\\[0\\]). - Violating transfer, size attribute (DATA\\[5:4\\]). '0': 8-bit transfer, '1': 16 bits transfer, '2': 32-bit transfer."]
    #[inline(always)]
    pub fn data24(&self) -> Data24R {
        Data24R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:26 - Error source: '0': INSTR_OPC_ERROR (instruction FIFO decoder error). '1': INSTR_CC_ERROR (instruction FIFO decoder, VU CC error). '2': BUS_ERROR (bus master interface AHB-Lite bus error). '3': TR_AP_DETECT_ERROR. '4': TR_RC_DETECT_ERROR. '5': INSTR_DEV_KEY_ERROR. '6'-'7': Undefined."]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<ErrorStatus1Spec> {
        ValidW::new(self, 31)
    }
}
#[doc = "Error status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorStatus1Spec;
impl crate::RegisterSpec for ErrorStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_status1::R`](R) reader structure"]
impl crate::Readable for ErrorStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`error_status1::W`](W) writer structure"]
impl crate::Writable for ErrorStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR_STATUS1 to value 0"]
impl crate::Resettable for ErrorStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
