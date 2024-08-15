#[doc = "Register `CM0_CTL` reader"]
pub type R = crate::R<Cm0CtlSpec>;
#[doc = "Register `CM0_CTL` writer"]
pub type W = crate::W<Cm0CtlSpec>;
#[doc = "Field `SLV_STALL` reader - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
pub type SlvStallR = crate::BitReader;
#[doc = "Field `SLV_STALL` writer - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
pub type SlvStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
pub type VectkeystatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
    #[inline(always)]
    pub fn slv_stall(&self) -> SlvStallR {
        SlvStallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VectkeystatR {
        VectkeystatR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
    #[inline(always)]
    #[must_use]
    pub fn slv_stall(&mut self) -> SlvStallW<Cm0CtlSpec> {
        SlvStallW::new(self, 0)
    }
    #[doc = "Bit 1 - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<Cm0CtlSpec> {
        EnabledW::new(self, 1)
    }
}
#[doc = "CM0+ control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0CtlSpec;
impl crate::RegisterSpec for Cm0CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ctl::R`](R) reader structure"]
impl crate::Readable for Cm0CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_ctl::W`](W) writer structure"]
impl crate::Writable for Cm0CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_CTL to value 0xfa05_0002"]
impl crate::Resettable for Cm0CtlSpec {
    const RESET_VALUE: u32 = 0xfa05_0002;
}
