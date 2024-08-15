#[doc = "Register `MS_CTL` reader"]
pub type R = crate::R<MsCtlSpec>;
#[doc = "Register `MS_CTL` writer"]
pub type W = crate::W<MsCtlSpec>;
#[doc = "Field `PC` reader - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
pub type PcR = crate::FieldReader;
#[doc = "Field `PC` writer - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
pub type PcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PC_SAVED` reader - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
pub type PcSavedR = crate::FieldReader;
#[doc = "Field `PC_SAVED` writer - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
pub type PcSavedW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
    #[inline(always)]
    pub fn pc_saved(&self) -> PcSavedR {
        PcSavedR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PcW<MsCtlSpec> {
        PcW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
    #[inline(always)]
    #[must_use]
    pub fn pc_saved(&mut self) -> PcSavedW<MsCtlSpec> {
        PcSavedW::new(self, 16)
    }
}
#[doc = "Master control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsCtlSpec;
impl crate::RegisterSpec for MsCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_ctl::R`](R) reader structure"]
impl crate::Readable for MsCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ms_ctl::W`](W) writer structure"]
impl crate::Writable for MsCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MS_CTL to value 0"]
impl crate::Resettable for MsCtlSpec {
    const RESET_VALUE: u32 = 0;
}
