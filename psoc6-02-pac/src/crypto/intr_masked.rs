#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `INSTR_FF_LEVEL` reader - Logical and of corresponding request and mask bits."]
pub type InstrFfLevelR = crate::BitReader;
#[doc = "Field `INSTR_FF_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type InstrFfOverflowR = crate::BitReader;
#[doc = "Field `TR_INITIALIZED` reader - Logical and of corresponding request and mask bits."]
pub type TrInitializedR = crate::BitReader;
#[doc = "Field `TR_DATA_AVAILABLE` reader - Logical and of corresponding request and mask bits."]
pub type TrDataAvailableR = crate::BitReader;
#[doc = "Field `PR_DATA_AVAILABLE` reader - Logical and of corresponding request and mask bits."]
pub type PrDataAvailableR = crate::BitReader;
#[doc = "Field `INSTR_OPC_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type InstrOpcErrorR = crate::BitReader;
#[doc = "Field `INSTR_CC_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type InstrCcErrorR = crate::BitReader;
#[doc = "Field `BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type BusErrorR = crate::BitReader;
#[doc = "Field `TR_AP_DETECT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type TrApDetectErrorR = crate::BitReader;
#[doc = "Field `TR_RC_DETECT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type TrRcDetectErrorR = crate::BitReader;
#[doc = "Field `INSTR_DEV_KEY_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type InstrDevKeyErrorR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_ff_level(&self) -> InstrFfLevelR {
        InstrFfLevelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_ff_overflow(&self) -> InstrFfOverflowR {
        InstrFfOverflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_initialized(&self) -> TrInitializedR {
        TrInitializedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_data_available(&self) -> TrDataAvailableR {
        TrDataAvailableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn pr_data_available(&self) -> PrDataAvailableR {
        PrDataAvailableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_opc_error(&self) -> InstrOpcErrorR {
        InstrOpcErrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_cc_error(&self) -> InstrCcErrorR {
        InstrCcErrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn bus_error(&self) -> BusErrorR {
        BusErrorR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_ap_detect_error(&self) -> TrApDetectErrorR {
        TrApDetectErrorR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_rc_detect_error(&self) -> TrRcDetectErrorR {
        TrRcDetectErrorR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_dev_key_error(&self) -> InstrDevKeyErrorR {
        InstrDevKeyErrorR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskedSpec;
impl crate::RegisterSpec for IntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_masked::R`](R) reader structure"]
impl crate::Readable for IntrMaskedSpec {}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for IntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
