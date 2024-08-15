#[doc = "Register `FLASH_CTL` reader"]
pub type R = crate::R<FlashCtlSpec>;
#[doc = "Register `FLASH_CTL` writer"]
pub type W = crate::W<FlashCtlSpec>;
#[doc = "Field `MAIN_WS` reader - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
pub type MainWsR = crate::FieldReader;
#[doc = "Field `MAIN_WS` writer - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
pub type MainWsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAIN_MAP` reader - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
pub type MainMapR = crate::BitReader;
#[doc = "Field `MAIN_MAP` writer - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
pub type MainMapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MAP` reader - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
pub type WorkMapR = crate::BitReader;
#[doc = "Field `WORK_MAP` writer - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
pub type WorkMapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_BANK_MODE` reader - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
pub type MainBankModeR = crate::BitReader;
#[doc = "Field `MAIN_BANK_MODE` writer - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
pub type MainBankModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_BANK_MODE` reader - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
pub type WorkBankModeR = crate::BitReader;
#[doc = "Field `WORK_BANK_MODE` writer - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
pub type WorkBankModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_ECC_EN` reader - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type MainEccEnR = crate::BitReader;
#[doc = "Field `MAIN_ECC_EN` writer - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type MainEccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_ECC_INJ_EN` reader - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type MainEccInjEnR = crate::BitReader;
#[doc = "Field `MAIN_ECC_INJ_EN` writer - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type MainEccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_ERR_SILENT` reader - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
pub type MainErrSilentR = crate::BitReader;
#[doc = "Field `MAIN_ERR_SILENT` writer - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
pub type MainErrSilentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_ECC_EN` reader - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type WorkEccEnR = crate::BitReader;
#[doc = "Field `WORK_ECC_EN` writer - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type WorkEccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_ECC_INJ_EN` reader - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type WorkEccInjEnR = crate::BitReader;
#[doc = "Field `WORK_ECC_INJ_EN` writer - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type WorkEccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_ERR_SILENT` reader - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
pub type WorkErrSilentR = crate::BitReader;
#[doc = "Field `WORK_ERR_SILENT` writer - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
pub type WorkErrSilentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&self) -> MainWsR {
        MainWsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn main_map(&self) -> MainMapR {
        MainMapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn work_map(&self) -> WorkMapR {
        WorkMapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn main_bank_mode(&self) -> MainBankModeR {
        MainBankModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn work_bank_mode(&self) -> WorkBankModeR {
        WorkBankModeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn main_ecc_en(&self) -> MainEccEnR {
        MainEccEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn main_ecc_inj_en(&self) -> MainEccInjEnR {
        MainEccInjEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    pub fn main_err_silent(&self) -> MainErrSilentR {
        MainErrSilentR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn work_ecc_en(&self) -> WorkEccEnR {
        WorkEccEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn work_ecc_inj_en(&self) -> WorkEccInjEnR {
        WorkEccInjEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    pub fn work_err_silent(&self) -> WorkErrSilentR {
        WorkErrSilentR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    #[must_use]
    pub fn main_ws(&mut self) -> MainWsW<FlashCtlSpec> {
        MainWsW::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    #[must_use]
    pub fn main_map(&mut self) -> MainMapW<FlashCtlSpec> {
        MainMapW::new(self, 8)
    }
    #[doc = "Bit 9 - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    #[must_use]
    pub fn work_map(&mut self) -> WorkMapW<FlashCtlSpec> {
        WorkMapW::new(self, 9)
    }
    #[doc = "Bit 12 - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    #[must_use]
    pub fn main_bank_mode(&mut self) -> MainBankModeW<FlashCtlSpec> {
        MainBankModeW::new(self, 12)
    }
    #[doc = "Bit 13 - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    #[must_use]
    pub fn work_bank_mode(&mut self) -> WorkBankModeW<FlashCtlSpec> {
        WorkBankModeW::new(self, 13)
    }
    #[doc = "Bit 16 - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn main_ecc_en(&mut self) -> MainEccEnW<FlashCtlSpec> {
        MainEccEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    #[must_use]
    pub fn main_ecc_inj_en(&mut self) -> MainEccInjEnW<FlashCtlSpec> {
        MainEccInjEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    #[must_use]
    pub fn main_err_silent(&mut self) -> MainErrSilentW<FlashCtlSpec> {
        MainErrSilentW::new(self, 18)
    }
    #[doc = "Bit 20 - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn work_ecc_en(&mut self) -> WorkEccEnW<FlashCtlSpec> {
        WorkEccEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    #[must_use]
    pub fn work_ecc_inj_en(&mut self) -> WorkEccInjEnW<FlashCtlSpec> {
        WorkEccInjEnW::new(self, 21)
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    #[must_use]
    pub fn work_err_silent(&mut self) -> WorkErrSilentW<FlashCtlSpec> {
        WorkErrSilentW::new(self, 22)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashCtlSpec;
impl crate::RegisterSpec for FlashCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ctl::R`](R) reader structure"]
impl crate::Readable for FlashCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_ctl::W`](W) writer structure"]
impl crate::Writable for FlashCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_CTL to value 0x0011_0000"]
impl crate::Resettable for FlashCtlSpec {
    const RESET_VALUE: u32 = 0x0011_0000;
}
