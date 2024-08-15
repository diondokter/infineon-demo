#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TX_ENABLED` reader - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub type TxEnabledR = crate::BitReader;
#[doc = "Field `TX_ENABLED` writer - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub type TxEnabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENABLED` reader - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub type RxEnabledR = crate::BitReader;
#[doc = "Field `RX_ENABLED` writer - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub type RxEnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&self) -> TxEnabledR {
        TxEnabledR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&self) -> RxEnabledR {
        RxEnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tx_enabled(&mut self) -> TxEnabledW<CtlSpec> {
        TxEnabledW::new(self, 30)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_enabled(&mut self) -> RxEnabledW<CtlSpec> {
        RxEnabledW::new(self, 31)
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
