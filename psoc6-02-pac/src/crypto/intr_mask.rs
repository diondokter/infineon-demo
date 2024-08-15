#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `INSTR_FF_LEVEL` reader - Mask bit for corresponding field in interrupt request register."]
pub type InstrFfLevelR = crate::BitReader;
#[doc = "Field `INSTR_FF_LEVEL` writer - Mask bit for corresponding field in interrupt request register."]
pub type InstrFfLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_FF_OVERFLOW` reader - Mask bit for corresponding field in interrupt request register."]
pub type InstrFfOverflowR = crate::BitReader;
#[doc = "Field `INSTR_FF_OVERFLOW` writer - Mask bit for corresponding field in interrupt request register."]
pub type InstrFfOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_INITIALIZED` reader - Mask bit for corresponding field in interrupt request register."]
pub type TrInitializedR = crate::BitReader;
#[doc = "Field `TR_INITIALIZED` writer - Mask bit for corresponding field in interrupt request register."]
pub type TrInitializedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_DATA_AVAILABLE` reader - Mask bit for corresponding field in interrupt request register."]
pub type TrDataAvailableR = crate::BitReader;
#[doc = "Field `TR_DATA_AVAILABLE` writer - Mask bit for corresponding field in interrupt request register."]
pub type TrDataAvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR_DATA_AVAILABLE` reader - Mask bit for corresponding field in interrupt request register."]
pub type PrDataAvailableR = crate::BitReader;
#[doc = "Field `PR_DATA_AVAILABLE` writer - Mask bit for corresponding field in interrupt request register."]
pub type PrDataAvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_OPC_ERROR` reader - Mask bit for corresponding field in interrupt request register."]
pub type InstrOpcErrorR = crate::BitReader;
#[doc = "Field `INSTR_OPC_ERROR` writer - Mask bit for corresponding field in interrupt request register."]
pub type InstrOpcErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_CC_ERROR` reader - Mask bit for corresponding field in interrupt request register."]
pub type InstrCcErrorR = crate::BitReader;
#[doc = "Field `INSTR_CC_ERROR` writer - Mask bit for corresponding field in interrupt request register."]
pub type InstrCcErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERROR` reader - Mask bit for corresponding field in interrupt request register."]
pub type BusErrorR = crate::BitReader;
#[doc = "Field `BUS_ERROR` writer - Mask bit for corresponding field in interrupt request register."]
pub type BusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_AP_DETECT_ERROR` reader - Mask bit for corresponding field in interrupt request register."]
pub type TrApDetectErrorR = crate::BitReader;
#[doc = "Field `TR_AP_DETECT_ERROR` writer - Mask bit for corresponding field in interrupt request register."]
pub type TrApDetectErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_RC_DETECT_ERROR` reader - Mask bit for corresponding field in interrupt request register."]
pub type TrRcDetectErrorR = crate::BitReader;
#[doc = "Field `TR_RC_DETECT_ERROR` writer - Mask bit for corresponding field in interrupt request register."]
pub type TrRcDetectErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_DEV_KEY_ERROR` reader - Mask bit for corresponding field in interrupt request register."]
pub type InstrDevKeyErrorR = crate::BitReader;
#[doc = "Field `INSTR_DEV_KEY_ERROR` writer - Mask bit for corresponding field in interrupt request register."]
pub type InstrDevKeyErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn instr_ff_level(&self) -> InstrFfLevelR {
        InstrFfLevelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn instr_ff_overflow(&self) -> InstrFfOverflowR {
        InstrFfOverflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn tr_initialized(&self) -> TrInitializedR {
        TrInitializedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn tr_data_available(&self) -> TrDataAvailableR {
        TrDataAvailableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn pr_data_available(&self) -> PrDataAvailableR {
        PrDataAvailableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn instr_opc_error(&self) -> InstrOpcErrorR {
        InstrOpcErrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn instr_cc_error(&self) -> InstrCcErrorR {
        InstrCcErrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn bus_error(&self) -> BusErrorR {
        BusErrorR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn tr_ap_detect_error(&self) -> TrApDetectErrorR {
        TrApDetectErrorR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn tr_rc_detect_error(&self) -> TrRcDetectErrorR {
        TrRcDetectErrorR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    pub fn instr_dev_key_error(&self) -> InstrDevKeyErrorR {
        InstrDevKeyErrorR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn instr_ff_level(&mut self) -> InstrFfLevelW<IntrMaskSpec> {
        InstrFfLevelW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn instr_ff_overflow(&mut self) -> InstrFfOverflowW<IntrMaskSpec> {
        InstrFfOverflowW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_initialized(&mut self) -> TrInitializedW<IntrMaskSpec> {
        TrInitializedW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_data_available(&mut self) -> TrDataAvailableW<IntrMaskSpec> {
        TrDataAvailableW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn pr_data_available(&mut self) -> PrDataAvailableW<IntrMaskSpec> {
        PrDataAvailableW::new(self, 4)
    }
    #[doc = "Bit 16 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn instr_opc_error(&mut self) -> InstrOpcErrorW<IntrMaskSpec> {
        InstrOpcErrorW::new(self, 16)
    }
    #[doc = "Bit 17 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn instr_cc_error(&mut self) -> InstrCcErrorW<IntrMaskSpec> {
        InstrCcErrorW::new(self, 17)
    }
    #[doc = "Bit 18 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn bus_error(&mut self) -> BusErrorW<IntrMaskSpec> {
        BusErrorW::new(self, 18)
    }
    #[doc = "Bit 19 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_ap_detect_error(&mut self) -> TrApDetectErrorW<IntrMaskSpec> {
        TrApDetectErrorW::new(self, 19)
    }
    #[doc = "Bit 20 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rc_detect_error(&mut self) -> TrRcDetectErrorW<IntrMaskSpec> {
        TrRcDetectErrorW::new(self, 20)
    }
    #[doc = "Bit 21 - Mask bit for corresponding field in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn instr_dev_key_error(&mut self) -> InstrDevKeyErrorW<IntrMaskSpec> {
        InstrDevKeyErrorW::new(self, 21)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
