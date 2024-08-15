#[doc = "Register `INJ_CHAN_CONFIG` reader"]
pub type R = crate::R<InjChanConfigSpec>;
#[doc = "Register `INJ_CHAN_CONFIG` writer"]
pub type W = crate::W<InjChanConfigSpec>;
#[doc = "Field `INJ_PIN_ADDR` reader - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
pub type InjPinAddrR = crate::FieldReader;
#[doc = "Field `INJ_PIN_ADDR` writer - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
pub type InjPinAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Address of the port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InjPortAddr {
    #[doc = "0: SARMUX pins."]
    Sarmux = 0,
    #[doc = "1: CTB0"]
    Ctb0 = 1,
    #[doc = "2: CTB1"]
    Ctb1 = 2,
    #[doc = "3: CTB2"]
    Ctb2 = 3,
    #[doc = "4: CTB3"]
    Ctb3 = 4,
    #[doc = "6: AROUTE virtual port"]
    ArouteVirt = 6,
    #[doc = "7: SARMUX virtual port"]
    SarmuxVirt = 7,
}
impl From<InjPortAddr> for u8 {
    #[inline(always)]
    fn from(variant: InjPortAddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for InjPortAddr {
    type Ux = u8;
}
impl crate::IsEnum for InjPortAddr {}
#[doc = "Field `INJ_PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel."]
pub type InjPortAddrR = crate::FieldReader<InjPortAddr>;
impl InjPortAddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<InjPortAddr> {
        match self.bits {
            0 => Some(InjPortAddr::Sarmux),
            1 => Some(InjPortAddr::Ctb0),
            2 => Some(InjPortAddr::Ctb1),
            3 => Some(InjPortAddr::Ctb2),
            4 => Some(InjPortAddr::Ctb3),
            6 => Some(InjPortAddr::ArouteVirt),
            7 => Some(InjPortAddr::SarmuxVirt),
            _ => None,
        }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == InjPortAddr::Sarmux
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        *self == InjPortAddr::Ctb0
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        *self == InjPortAddr::Ctb1
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        *self == InjPortAddr::Ctb2
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        *self == InjPortAddr::Ctb3
    }
    #[doc = "AROUTE virtual port"]
    #[inline(always)]
    pub fn is_aroute_virt(&self) -> bool {
        *self == InjPortAddr::ArouteVirt
    }
    #[doc = "SARMUX virtual port"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == InjPortAddr::SarmuxVirt
    }
}
#[doc = "Field `INJ_PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel."]
pub type InjPortAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3, InjPortAddr>;
impl<'a, REG> InjPortAddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut crate::W<REG> {
        self.variant(InjPortAddr::Sarmux)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut crate::W<REG> {
        self.variant(InjPortAddr::Ctb0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut crate::W<REG> {
        self.variant(InjPortAddr::Ctb1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut crate::W<REG> {
        self.variant(InjPortAddr::Ctb2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut crate::W<REG> {
        self.variant(InjPortAddr::Ctb3)
    }
    #[doc = "AROUTE virtual port"]
    #[inline(always)]
    pub fn aroute_virt(self) -> &'a mut crate::W<REG> {
        self.variant(InjPortAddr::ArouteVirt)
    }
    #[doc = "SARMUX virtual port"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut crate::W<REG> {
        self.variant(InjPortAddr::SarmuxVirt)
    }
}
#[doc = "Field `INJ_DIFFERENTIAL_EN` reader - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
pub type InjDifferentialEnR = crate::BitReader;
#[doc = "Field `INJ_DIFFERENTIAL_EN` writer - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
pub type InjDifferentialEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type InjAvgEnR = crate::BitReader;
#[doc = "Field `INJ_AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type InjAvgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_SAMPLE_TIME_SEL` reader - Injection sample time select: select which of the 4 global sample times to use for this channel"]
pub type InjSampleTimeSelR = crate::FieldReader;
#[doc = "Field `INJ_SAMPLE_TIME_SEL` writer - Injection sample time select: select which of the 4 global sample times to use for this channel"]
pub type InjSampleTimeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INJ_TAILGATING` reader - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
pub type InjTailgatingR = crate::BitReader;
#[doc = "Field `INJ_TAILGATING` writer - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
pub type InjTailgatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_START_EN` reader - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
pub type InjStartEnR = crate::BitReader;
#[doc = "Field `INJ_START_EN` writer - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
pub type InjStartEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn inj_pin_addr(&self) -> InjPinAddrR {
        InjPinAddrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn inj_port_addr(&self) -> InjPortAddrR {
        InjPortAddrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn inj_differential_en(&self) -> InjDifferentialEnR {
        InjDifferentialEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn inj_avg_en(&self) -> InjAvgEnR {
        InjAvgEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn inj_sample_time_sel(&self) -> InjSampleTimeSelR {
        InjSampleTimeSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn inj_tailgating(&self) -> InjTailgatingR {
        InjTailgatingR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn inj_start_en(&self) -> InjStartEnR {
        InjStartEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    #[must_use]
    pub fn inj_pin_addr(&mut self) -> InjPinAddrW<InjChanConfigSpec> {
        InjPinAddrW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    #[must_use]
    pub fn inj_port_addr(&mut self) -> InjPortAddrW<InjChanConfigSpec> {
        InjPortAddrW::new(self, 4)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    #[must_use]
    pub fn inj_differential_en(&mut self) -> InjDifferentialEnW<InjChanConfigSpec> {
        InjDifferentialEnW::new(self, 8)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    #[must_use]
    pub fn inj_avg_en(&mut self) -> InjAvgEnW<InjChanConfigSpec> {
        InjAvgEnW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    #[must_use]
    pub fn inj_sample_time_sel(&mut self) -> InjSampleTimeSelW<InjChanConfigSpec> {
        InjSampleTimeSelW::new(self, 12)
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    #[must_use]
    pub fn inj_tailgating(&mut self) -> InjTailgatingW<InjChanConfigSpec> {
        InjTailgatingW::new(self, 30)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn inj_start_en(&mut self) -> InjStartEnW<InjChanConfigSpec> {
        InjStartEnW::new(self, 31)
    }
}
#[doc = "Injection channel configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inj_chan_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inj_chan_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InjChanConfigSpec;
impl crate::RegisterSpec for InjChanConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inj_chan_config::R`](R) reader structure"]
impl crate::Readable for InjChanConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`inj_chan_config::W`](W) writer structure"]
impl crate::Writable for InjChanConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INJ_CHAN_CONFIG to value 0"]
impl crate::Resettable for InjChanConfigSpec {
    const RESET_VALUE: u32 = 0;
}
