#[doc = "Register `PWR_BUCK_CTL` reader"]
pub type R = crate::R<PwrBuckCtlSpec>;
#[doc = "Register `PWR_BUCK_CTL` writer"]
pub type W = crate::W<PwrBuckCtlSpec>;
#[doc = "Field `BUCK_OUT1_SEL` reader - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
pub type BuckOut1SelR = crate::FieldReader;
#[doc = "Field `BUCK_OUT1_SEL` writer - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
pub type BuckOut1SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUCK_EN` reader - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type BuckEnR = crate::BitReader;
#[doc = "Field `BUCK_EN` writer - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type BuckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUCK_OUT1_EN` reader - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1. TRM must follow the SAS."]
pub type BuckOut1EnR = crate::BitReader;
#[doc = "Field `BUCK_OUT1_EN` writer - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1. TRM must follow the SAS."]
pub type BuckOut1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub fn buck_out1_sel(&self) -> BuckOut1SelR {
        BuckOut1SelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn buck_en(&self) -> BuckEnR {
        BuckEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1. TRM must follow the SAS."]
    #[inline(always)]
    pub fn buck_out1_en(&self) -> BuckOut1EnR {
        BuckOut1EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    #[must_use]
    pub fn buck_out1_sel(&mut self) -> BuckOut1SelW<PwrBuckCtlSpec> {
        BuckOut1SelW::new(self, 0)
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn buck_en(&mut self) -> BuckEnW<PwrBuckCtlSpec> {
        BuckEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1. TRM must follow the SAS."]
    #[inline(always)]
    #[must_use]
    pub fn buck_out1_en(&mut self) -> BuckOut1EnW<PwrBuckCtlSpec> {
        BuckOut1EnW::new(self, 31)
    }
}
#[doc = "Buck Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_buck_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_buck_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrBuckCtlSpec;
impl crate::RegisterSpec for PwrBuckCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_buck_ctl::R`](R) reader structure"]
impl crate::Readable for PwrBuckCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_buck_ctl::W`](W) writer structure"]
impl crate::Writable for PwrBuckCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_BUCK_CTL to value 0x05"]
impl crate::Resettable for PwrBuckCtlSpec {
    const RESET_VALUE: u32 = 0x05;
}
