#[doc = "Register `PWR_BUCK_CTL2` reader"]
pub type R = crate::R<PwrBuckCtl2Spec>;
#[doc = "Register `PWR_BUCK_CTL2` writer"]
pub type W = crate::W<PwrBuckCtl2Spec>;
#[doc = "Field `BUCK_OUT2_SEL` reader - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
pub type BuckOut2SelR = crate::FieldReader;
#[doc = "Field `BUCK_OUT2_SEL` writer - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
pub type BuckOut2SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUCK_OUT2_HW_SEL` reader - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
pub type BuckOut2HwSelR = crate::BitReader;
#[doc = "Field `BUCK_OUT2_HW_SEL` writer - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
pub type BuckOut2HwSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUCK_OUT2_EN` reader - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
pub type BuckOut2EnR = crate::BitReader;
#[doc = "Field `BUCK_OUT2_EN` writer - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
pub type BuckOut2EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    pub fn buck_out2_sel(&self) -> BuckOut2SelR {
        BuckOut2SelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 30 - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    pub fn buck_out2_hw_sel(&self) -> BuckOut2HwSelR {
        BuckOut2HwSelR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    pub fn buck_out2_en(&self) -> BuckOut2EnR {
        BuckOut2EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    #[must_use]
    pub fn buck_out2_sel(&mut self) -> BuckOut2SelW<PwrBuckCtl2Spec> {
        BuckOut2SelW::new(self, 0)
    }
    #[doc = "Bit 30 - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    #[must_use]
    pub fn buck_out2_hw_sel(&mut self) -> BuckOut2HwSelW<PwrBuckCtl2Spec> {
        BuckOut2HwSelW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    #[must_use]
    pub fn buck_out2_en(&mut self) -> BuckOut2EnW<PwrBuckCtl2Spec> {
        BuckOut2EnW::new(self, 31)
    }
}
#[doc = "Buck Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_buck_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_buck_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrBuckCtl2Spec;
impl crate::RegisterSpec for PwrBuckCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_buck_ctl2::R`](R) reader structure"]
impl crate::Readable for PwrBuckCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_buck_ctl2::W`](W) writer structure"]
impl crate::Writable for PwrBuckCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_BUCK_CTL2 to value 0"]
impl crate::Resettable for PwrBuckCtl2Spec {
    const RESET_VALUE: u32 = 0;
}
