#[doc = "Register `PWR_TRIM_PWRSYS_CTL` reader"]
pub type R = crate::R<PwrTrimPwrsysCtlSpec>;
#[doc = "Register `PWR_TRIM_PWRSYS_CTL` writer"]
pub type W = crate::W<PwrTrimPwrsysCtlSpec>;
#[doc = "Field `ACT_REG_TRIM` reader - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
pub type ActRegTrimR = crate::FieldReader;
#[doc = "Field `ACT_REG_TRIM` writer - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
pub type ActRegTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ACT_REG_BOOST` reader - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type ActRegBoostR = crate::FieldReader;
#[doc = "Field `ACT_REG_BOOST` writer - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type ActRegBoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
    #[inline(always)]
    pub fn act_reg_trim(&self) -> ActRegTrimR {
        ActRegTrimR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn act_reg_boost(&self) -> ActRegBoostR {
        ActRegBoostR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn act_reg_trim(&mut self) -> ActRegTrimW<PwrTrimPwrsysCtlSpec> {
        ActRegTrimW::new(self, 0)
    }
    #[doc = "Bits 30:31 - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn act_reg_boost(&mut self) -> ActRegBoostW<PwrTrimPwrsysCtlSpec> {
        ActRegBoostW::new(self, 30)
    }
}
#[doc = "Power System Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_pwrsys_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_pwrsys_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrTrimPwrsysCtlSpec;
impl crate::RegisterSpec for PwrTrimPwrsysCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_pwrsys_ctl::R`](R) reader structure"]
impl crate::Readable for PwrTrimPwrsysCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_pwrsys_ctl::W`](W) writer structure"]
impl crate::Writable for PwrTrimPwrsysCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_PWRSYS_CTL to value 0x17"]
impl crate::Resettable for PwrTrimPwrsysCtlSpec {
    const RESET_VALUE: u32 = 0x17;
}
