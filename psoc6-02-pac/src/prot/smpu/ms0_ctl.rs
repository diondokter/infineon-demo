#[doc = "Register `MS0_CTL` reader"]
pub type R = crate::R<Ms0CtlSpec>;
#[doc = "Register `MS0_CTL` writer"]
pub type W = crate::W<Ms0CtlSpec>;
#[doc = "Field `P` reader - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIO` reader - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC_MASK_0` reader - Protection context mask for protection context '0'. This field is a constant '0': - PC_MASK_0 is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to '0' and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed."]
pub type PcMask0R = crate::BitReader;
#[doc = "Field `PC_MASK_15_TO_1` reader - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
pub type PcMask15To1R = crate::FieldReader<u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
pub type PcMask15To1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Protection context mask for protection context '0'. This field is a constant '0': - PC_MASK_0 is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to '0' and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PcMask0R {
        PcMask0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PcMask15To1R {
        PcMask15To1R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<Ms0CtlSpec> {
        PW::new(self, 0)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<Ms0CtlSpec> {
        NsW::new(self, 1)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<Ms0CtlSpec> {
        PrioW::new(self, 8)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn pc_mask_15_to_1(&mut self) -> PcMask15To1W<Ms0CtlSpec> {
        PcMask15To1W::new(self, 17)
    }
}
#[doc = "Master 0 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms0_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms0_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ms0CtlSpec;
impl crate::RegisterSpec for Ms0CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms0_ctl::R`](R) reader structure"]
impl crate::Readable for Ms0CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ms0_ctl::W`](W) writer structure"]
impl crate::Writable for Ms0CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MS0_CTL to value 0x0303"]
impl crate::Resettable for Ms0CtlSpec {
    const RESET_VALUE: u32 = 0x0303;
}
