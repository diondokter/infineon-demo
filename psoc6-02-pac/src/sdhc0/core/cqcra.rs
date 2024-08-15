#[doc = "Register `CQCRA` reader"]
pub type R = crate::R<CqcraSpec>;
#[doc = "Field `CMD_RESP_ARG` reader - Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
pub type CmdRespArgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
    #[inline(always)]
    pub fn cmd_resp_arg(&self) -> CmdRespArgR {
        CmdRespArgR::new(self.bits)
    }
}
#[doc = "CQ Command response argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcra::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcraSpec;
impl crate::RegisterSpec for CqcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcra::R`](R) reader structure"]
impl crate::Readable for CqcraSpec {}
#[doc = "`reset()` method sets CQCRA to value 0"]
impl crate::Resettable for CqcraSpec {
    const RESET_VALUE: u32 = 0;
}
