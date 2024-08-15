#[doc = "Register `FLOW_CTL` reader"]
pub type R = crate::R<FlowCtlSpec>;
#[doc = "Register `FLOW_CTL` writer"]
pub type W = crate::W<FlowCtlSpec>;
#[doc = "Field `EP1_ERR_RESP` reader - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub type Ep1ErrRespR = crate::BitReader;
#[doc = "Field `EP1_ERR_RESP` writer - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub type Ep1ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_ERR_RESP` reader - End Point 2 error response"]
pub type Ep2ErrRespR = crate::BitReader;
#[doc = "Field `EP2_ERR_RESP` writer - End Point 2 error response"]
pub type Ep2ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_ERR_RESP` reader - End Point 3 error response"]
pub type Ep3ErrRespR = crate::BitReader;
#[doc = "Field `EP3_ERR_RESP` writer - End Point 3 error response"]
pub type Ep3ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_ERR_RESP` reader - End Point 4 error response"]
pub type Ep4ErrRespR = crate::BitReader;
#[doc = "Field `EP4_ERR_RESP` writer - End Point 4 error response"]
pub type Ep4ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_ERR_RESP` reader - End Point 5 error response"]
pub type Ep5ErrRespR = crate::BitReader;
#[doc = "Field `EP5_ERR_RESP` writer - End Point 5 error response"]
pub type Ep5ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_ERR_RESP` reader - End Point 6 error response"]
pub type Ep6ErrRespR = crate::BitReader;
#[doc = "Field `EP6_ERR_RESP` writer - End Point 6 error response"]
pub type Ep6ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_ERR_RESP` reader - End Point 7 error response"]
pub type Ep7ErrRespR = crate::BitReader;
#[doc = "Field `EP7_ERR_RESP` writer - End Point 7 error response"]
pub type Ep7ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_ERR_RESP` reader - End Point 8 error response"]
pub type Ep8ErrRespR = crate::BitReader;
#[doc = "Field `EP8_ERR_RESP` writer - End Point 8 error response"]
pub type Ep8ErrRespW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&self) -> Ep1ErrRespR {
        Ep1ErrRespR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&self) -> Ep2ErrRespR {
        Ep2ErrRespR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&self) -> Ep3ErrRespR {
        Ep3ErrRespR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&self) -> Ep4ErrRespR {
        Ep4ErrRespR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&self) -> Ep5ErrRespR {
        Ep5ErrRespR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&self) -> Ep6ErrRespR {
        Ep6ErrRespR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&self) -> Ep7ErrRespR {
        Ep7ErrRespR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&self) -> Ep8ErrRespR {
        Ep8ErrRespR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_err_resp(&mut self) -> Ep1ErrRespW<FlowCtlSpec> {
        Ep1ErrRespW::new(self, 0)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_err_resp(&mut self) -> Ep2ErrRespW<FlowCtlSpec> {
        Ep2ErrRespW::new(self, 1)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_err_resp(&mut self) -> Ep3ErrRespW<FlowCtlSpec> {
        Ep3ErrRespW::new(self, 2)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_err_resp(&mut self) -> Ep4ErrRespW<FlowCtlSpec> {
        Ep4ErrRespW::new(self, 3)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_err_resp(&mut self) -> Ep5ErrRespW<FlowCtlSpec> {
        Ep5ErrRespW::new(self, 4)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_err_resp(&mut self) -> Ep6ErrRespW<FlowCtlSpec> {
        Ep6ErrRespW::new(self, 5)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_err_resp(&mut self) -> Ep7ErrRespW<FlowCtlSpec> {
        Ep7ErrRespW::new(self, 6)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_err_resp(&mut self) -> Ep8ErrRespW<FlowCtlSpec> {
        Ep8ErrRespW::new(self, 7)
    }
}
#[doc = "Flow Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flow_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlowCtlSpec;
impl crate::RegisterSpec for FlowCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flow_ctl::R`](R) reader structure"]
impl crate::Readable for FlowCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`flow_ctl::W`](W) writer structure"]
impl crate::Writable for FlowCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLOW_CTL to value 0"]
impl crate::Resettable for FlowCtlSpec {
    const RESET_VALUE: u32 = 0;
}
