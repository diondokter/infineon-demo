#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `P` reader - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC` reader - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the PC field for the protection context. There is one exception: the LOAD_DEV_KEY instruction IP master transactions are always performed with protection context '0'."]
pub type PcR = crate::FieldReader;
#[doc = "Field `PC` writer - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the PC field for the protection context. There is one exception: the LOAD_DEV_KEY instruction IP master transactions are always performed with protection context '0'."]
pub type PcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
pub type EccInjEnR = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
pub type EccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IP enable: '0': Disabled. All non-retention registers (command and status registers, instruct FIFO, internal component state machines) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled. When the IP is enabled, the IP register buffer is set to '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enabled {
    #[doc = "0: N/A"]
    Disabled = 0,
    #[doc = "1: N/A"]
    Enabled = 1,
}
impl From<Enabled> for bool {
    #[inline(always)]
    fn from(variant: Enabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLED` reader - IP enable: '0': Disabled. All non-retention registers (command and status registers, instruct FIFO, internal component state machines) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled. When the IP is enabled, the IP register buffer is set to '0'."]
pub type EnabledR = crate::BitReader<Enabled>;
impl EnabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enabled {
        match self.bits {
            false => Enabled::Disabled,
            true => Enabled::Enabled,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enabled::Disabled
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enabled::Enabled
    }
}
#[doc = "Field `ENABLED` writer - IP enable: '0': Disabled. All non-retention registers (command and status registers, instruct FIFO, internal component state machines) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled. When the IP is enabled, the IP register buffer is set to '0'."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG, Enabled>;
impl<'a, REG> EnabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabled::Disabled)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabled::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the PC field for the protection context. There is one exception: the LOAD_DEV_KEY instruction IP master transactions are always performed with protection context '0'."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> EccInjEnR {
        EccInjEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers (command and status registers, instruct FIFO, internal component state machines) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled. When the IP is enabled, the IP register buffer is set to '0'."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<CtlSpec> {
        PW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<CtlSpec> {
        NsW::new(self, 1)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All IP master transactions use the PC field for the protection context. There is one exception: the LOAD_DEV_KEY instruction IP master transactions are always performed with protection context '0'."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PcW<CtlSpec> {
        PcW::new(self, 4)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<CtlSpec> {
        EccEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> EccInjEnW<CtlSpec> {
        EccInjEnW::new(self, 17)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers (command and status registers, instruct FIFO, internal component state machines) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled. When the IP is enabled, the IP register buffer is set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0001_0002"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0001_0002;
}
