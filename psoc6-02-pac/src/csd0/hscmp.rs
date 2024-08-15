#[doc = "Register `HSCMP` reader"]
pub type R = crate::R<HscmpSpec>;
#[doc = "Register `HSCMP` writer"]
pub type W = crate::W<HscmpSpec>;
#[doc = "High Speed Comparator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HscmpEn {
    #[doc = "0: Disable comparator, output is zero"]
    Off = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    On = 1,
}
impl From<HscmpEn> for bool {
    #[inline(always)]
    fn from(variant: HscmpEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCMP_EN` reader - High Speed Comparator enable"]
pub type HscmpEnR = crate::BitReader<HscmpEn>;
impl HscmpEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HscmpEn {
        match self.bits {
            false => HscmpEn::Off,
            true => HscmpEn::On,
        }
    }
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HscmpEn::Off
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HscmpEn::On
    }
}
#[doc = "Field `HSCMP_EN` writer - High Speed Comparator enable"]
pub type HscmpEnW<'a, REG> = crate::BitWriter<'a, REG, HscmpEn>;
impl<'a, REG> HscmpEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HscmpEn::Off)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HscmpEn::On)
    }
}
#[doc = "Field `HSCMP_INVERT` reader - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
pub type HscmpInvertR = crate::BitReader;
#[doc = "Field `HSCMP_INVERT` writer - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
pub type HscmpInvertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZ_EN` reader - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AzEnR = crate::BitReader;
#[doc = "Field `AZ_EN` writer - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AzEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    pub fn hscmp_en(&self) -> HscmpEnR {
        HscmpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn hscmp_invert(&self) -> HscmpInvertR {
        HscmpInvertR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&self) -> AzEnR {
        AzEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    #[must_use]
    pub fn hscmp_en(&mut self) -> HscmpEnW<HscmpSpec> {
        HscmpEnW::new(self, 0)
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn hscmp_invert(&mut self) -> HscmpInvertW<HscmpSpec> {
        HscmpInvertW::new(self, 4)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    #[must_use]
    pub fn az_en(&mut self) -> AzEnW<HscmpSpec> {
        AzEnW::new(self, 31)
    }
}
#[doc = "High Speed Comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hscmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hscmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HscmpSpec;
impl crate::RegisterSpec for HscmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hscmp::R`](R) reader structure"]
impl crate::Readable for HscmpSpec {}
#[doc = "`write(|w| ..)` method takes [`hscmp::W`](W) writer structure"]
impl crate::Writable for HscmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSCMP to value 0"]
impl crate::Resettable for HscmpSpec {
    const RESET_VALUE: u32 = 0;
}
