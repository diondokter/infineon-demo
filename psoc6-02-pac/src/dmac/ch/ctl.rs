#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `P` reader - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B` reader - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
pub type BR = crate::BitReader;
#[doc = "Field `B` writer - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
pub type BW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC` reader - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel uses the PC field for the protection context."]
pub type PcR = crate::FieldReader;
#[doc = "Field `PC` writer - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel uses the PC field for the protection context."]
pub type PcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIO` reader - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group and within this priority group, the following 'roundrobin' arbitration is applied. A 'round' consists of a contiguous sequence of channel activations, within this priority group, without any repetition. Within a round, higher priority is given to the lower channel indices. The notion of a round guarantees that within a group, higher channel indices do not yield to lower indices indefinitely."]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group and within this priority group, the following 'roundrobin' arbitration is applied. A 'round' consists of a contiguous sequence of channel activations, within this priority group, without any repetition. Within a round, higher priority is given to the lower channel indices. The notion of a round guarantees that within a group, higher channel indices do not yield to lower indices indefinitely."]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENABLED` reader - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' when an error interrupt cause is activated."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' when an error interrupt cause is activated."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group and within this priority group, the following 'roundrobin' arbitration is applied. A 'round' consists of a contiguous sequence of channel activations, within this priority group, without any repetition. Within a round, higher priority is given to the lower channel indices. The notion of a round guarantees that within a group, higher channel indices do not yield to lower indices indefinitely."]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' when an error interrupt cause is activated."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<CtlSpec> {
        PW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<CtlSpec> {
        NsW::new(self, 1)
    }
    #[doc = "Bit 2 - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> BW<CtlSpec> {
        BW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PcW<CtlSpec> {
        PcW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group and within this priority group, the following 'roundrobin' arbitration is applied. A 'round' consists of a contiguous sequence of channel activations, within this priority group, without any repetition. Within a round, higher priority is given to the lower channel indices. The notion of a round guarantees that within a group, higher channel indices do not yield to lower indices indefinitely."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<CtlSpec> {
        PrioW::new(self, 8)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' when an error interrupt cause is activated."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x02"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x02;
}
