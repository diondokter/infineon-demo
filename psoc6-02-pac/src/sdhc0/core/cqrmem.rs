#[doc = "Register `CQRMEM` reader"]
pub type R = crate::R<CqrmemSpec>;
#[doc = "Register `CQRMEM` writer"]
pub type W = crate::W<CqrmemSpec>;
#[doc = "Field `RESP_ERR_MASK` reader - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
pub type RespErrMaskR = crate::FieldReader<u32>;
#[doc = "Field `RESP_ERR_MASK` writer - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
pub type RespErrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
    #[inline(always)]
    pub fn resp_err_mask(&self) -> RespErrMaskR {
        RespErrMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_mask(&mut self) -> RespErrMaskW<CqrmemSpec> {
        RespErrMaskW::new(self, 0)
    }
}
#[doc = "Command response mode error mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqrmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqrmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqrmemSpec;
impl crate::RegisterSpec for CqrmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqrmem::R`](R) reader structure"]
impl crate::Readable for CqrmemSpec {}
#[doc = "`write(|w| ..)` method takes [`cqrmem::W`](W) writer structure"]
impl crate::Writable for CqrmemSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQRMEM to value 0xfdf9_a080"]
impl crate::Resettable for CqrmemSpec {
    const RESET_VALUE: u32 = 0xfdf9_a080;
}
