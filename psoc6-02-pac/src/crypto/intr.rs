#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `INSTR_FF_LEVEL` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
pub type InstrFfLevelR = crate::BitReader;
#[doc = "Field `INSTR_FF_LEVEL` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
pub type InstrFfLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_FF_OVERFLOW` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
pub type InstrFfOverflowR = crate::BitReader;
#[doc = "Field `INSTR_FF_OVERFLOW` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
pub type InstrFfOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_INITIALIZED` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
pub type TrInitializedR = crate::BitReader;
#[doc = "Field `TR_INITIALIZED` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
pub type TrInitializedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_DATA_AVAILABLE` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
pub type TrDataAvailableR = crate::BitReader;
#[doc = "Field `TR_DATA_AVAILABLE` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
pub type TrDataAvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR_DATA_AVAILABLE` reader - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
pub type PrDataAvailableR = crate::BitReader;
#[doc = "Field `PR_DATA_AVAILABLE` writer - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
pub type PrDataAvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_OPC_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type InstrOpcErrorR = crate::BitReader;
#[doc = "Field `INSTR_OPC_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type InstrOpcErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_CC_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type InstrCcErrorR = crate::BitReader;
#[doc = "Field `INSTR_CC_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type InstrCcErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type BusErrorR = crate::BitReader;
#[doc = "Field `BUS_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
pub type BusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_AP_DETECT_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
pub type TrApDetectErrorR = crate::BitReader;
#[doc = "Field `TR_AP_DETECT_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
pub type TrApDetectErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_RC_DETECT_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
pub type TrRcDetectErrorR = crate::BitReader;
#[doc = "Field `TR_RC_DETECT_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
pub type TrRcDetectErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTR_DEV_KEY_ERROR` reader - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
pub type InstrDevKeyErrorR = crate::BitReader;
#[doc = "Field `INSTR_DEV_KEY_ERROR` writer - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
pub type InstrDevKeyErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
    #[inline(always)]
    pub fn instr_ff_level(&self) -> InstrFfLevelR {
        InstrFfLevelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
    #[inline(always)]
    pub fn instr_ff_overflow(&self) -> InstrFfOverflowR {
        InstrFfOverflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
    #[inline(always)]
    pub fn tr_initialized(&self) -> TrInitializedR {
        TrInitializedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
    #[inline(always)]
    pub fn tr_data_available(&self) -> TrDataAvailableR {
        TrDataAvailableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
    #[inline(always)]
    pub fn pr_data_available(&self) -> PrDataAvailableR {
        PrDataAvailableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    pub fn instr_opc_error(&self) -> InstrOpcErrorR {
        InstrOpcErrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    pub fn instr_cc_error(&self) -> InstrCcErrorR {
        InstrCcErrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    pub fn bus_error(&self) -> BusErrorR {
        BusErrorR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
    #[inline(always)]
    pub fn tr_ap_detect_error(&self) -> TrApDetectErrorR {
        TrApDetectErrorR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
    #[inline(always)]
    pub fn tr_rc_detect_error(&self) -> TrRcDetectErrorR {
        TrRcDetectErrorR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
    #[inline(always)]
    pub fn instr_dev_key_error(&self) -> InstrDevKeyErrorR {
        InstrDevKeyErrorR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO event is activated."]
    #[inline(always)]
    #[must_use]
    pub fn instr_ff_level(&mut self) -> InstrFfLevelW<IntrSpec> {
        InstrFfLevelW::new(self, 0)
    }
    #[doc = "Bit 1 - This interrupt cause is activated (HW sets the field to '1') when the instruction FIFO overflows (an attempt is made to write to a full FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn instr_ff_overflow(&mut self) -> InstrFfOverflowW<IntrSpec> {
        InstrFfOverflowW::new(self, 1)
    }
    #[doc = "Bit 2 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator is initialized."]
    #[inline(always)]
    #[must_use]
    pub fn tr_initialized(&mut self) -> TrInitializedW<IntrSpec> {
        TrInitializedW::new(self, 2)
    }
    #[doc = "Bit 3 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator has generated a data value of the specified bit size."]
    #[inline(always)]
    #[must_use]
    pub fn tr_data_available(&mut self) -> TrDataAvailableW<IntrSpec> {
        TrDataAvailableW::new(self, 3)
    }
    #[doc = "Bit 4 - This interrupt cause is activated (HW sets the field to '1') when the pseudo random number generator has generated a data value."]
    #[inline(always)]
    #[must_use]
    pub fn pr_data_available(&mut self) -> PrDataAvailableW<IntrSpec> {
        PrDataAvailableW::new(self, 4)
    }
    #[doc = "Bit 16 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined operation code (opcode). When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn instr_opc_error(&mut self) -> InstrOpcErrorW<IntrSpec> {
        InstrOpcErrorW::new(self, 16)
    }
    #[doc = "Bit 17 - This interrupt cause is activated (HW sets the field to '1') when the instruction decoder encounters an instruction with a non-defined condition code. This error is only generated for VU instructions. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn instr_cc_error(&mut self) -> InstrCcErrorW<IntrSpec> {
        InstrCcErrorW::new(self, 17)
    }
    #[doc = "Bit 18 - This interrupt cause is activated (HW sets the field to '1') when a AHB-Lite bus error is observed on the AHB-Lite master interface. When the interrupt cause is activated, HW sets INSTR_FF_CTL.CLEAR to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn bus_error(&mut self) -> BusErrorW<IntrSpec> {
        BusErrorW::new(self, 18)
    }
    #[doc = "Bit 19 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a repetition of a specific bit value."]
    #[inline(always)]
    #[must_use]
    pub fn tr_ap_detect_error(&mut self) -> TrApDetectErrorW<IntrSpec> {
        TrApDetectErrorW::new(self, 19)
    }
    #[doc = "Bit 20 - This interrupt cause is activated (HW sets the field to '1') when the true random number generator monitor adaptive proportion test detects a disproportionate occurrence of a specific bit value."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rc_detect_error(&mut self) -> TrRcDetectErrorW<IntrSpec> {
        TrRcDetectErrorW::new(self, 20)
    }
    #[doc = "Bit 21 - This interrupt cause is activated (HW sets the field to '1') when the LOAD_DEV_KEY instruction tries to load a device key whose DEV_KEY_ADDR_CTL.VALID or DEV_KEY_CTL.ALLOWED is set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn instr_dev_key_error(&mut self) -> InstrDevKeyErrorW<IntrSpec> {
        InstrDevKeyErrorW::new(self, 21)
    }
}
#[doc = "Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
