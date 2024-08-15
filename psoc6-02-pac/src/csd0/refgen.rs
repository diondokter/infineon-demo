#[doc = "Register `REFGEN` reader"]
pub type R = crate::R<RefgenSpec>;
#[doc = "Register `REFGEN` writer"]
pub type W = crate::W<RefgenSpec>;
#[doc = "Reference Generator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefgenEn {
    #[doc = "0: Disable Reference Generator"]
    Off = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    On = 1,
}
impl From<RefgenEn> for bool {
    #[inline(always)]
    fn from(variant: RefgenEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGEN_EN` reader - Reference Generator Enable"]
pub type RefgenEnR = crate::BitReader<RefgenEn>;
impl RefgenEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefgenEn {
        match self.bits {
            false => RefgenEn::Off,
            true => RefgenEn::On,
        }
    }
    #[doc = "Disable Reference Generator"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RefgenEn::Off
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RefgenEn::On
    }
}
#[doc = "Field `REFGEN_EN` writer - Reference Generator Enable"]
pub type RefgenEnW<'a, REG> = crate::BitWriter<'a, REG, RefgenEn>;
impl<'a, REG> RefgenEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Reference Generator"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(RefgenEn::Off)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(RefgenEn::On)
    }
}
#[doc = "Field `BYPASS` reader - Bypass selected input reference unbuffered to Vrefhi"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - Bypass selected input reference unbuffered to Vrefhi"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA_EN` reader - Close Vdda switch to top of resistor string (or Vrefhi?)"]
pub type VddaEnR = crate::BitReader;
#[doc = "Field `VDDA_EN` writer - Close Vdda switch to top of resistor string (or Vrefhi?)"]
pub type VddaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES_EN` reader - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
pub type ResEnR = crate::BitReader;
#[doc = "Field `RES_EN` writer - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
pub type ResEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAIN` reader - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREFLO_SEL` reader - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
pub type VrefloSelR = crate::FieldReader;
#[doc = "Field `VREFLO_SEL` writer - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
pub type VrefloSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREFLO_INT` reader - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
pub type VrefloIntR = crate::BitReader;
#[doc = "Field `VREFLO_INT` writer - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
pub type VrefloIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    pub fn refgen_en(&self) -> RefgenEnR {
        RefgenEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn vdda_en(&self) -> VddaEnR {
        VddaEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn res_en(&self) -> ResEnR {
        ResEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn vreflo_sel(&self) -> VrefloSelR {
        VrefloSelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub fn vreflo_int(&self) -> VrefloIntR {
        VrefloIntR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refgen_en(&mut self) -> RefgenEnW<RefgenSpec> {
        RefgenEnW::new(self, 0)
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<RefgenSpec> {
        BypassW::new(self, 4)
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    #[must_use]
    pub fn vdda_en(&mut self) -> VddaEnW<RefgenSpec> {
        VddaEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    #[must_use]
    pub fn res_en(&mut self) -> ResEnW<RefgenSpec> {
        ResEnW::new(self, 6)
    }
    #[doc = "Bits 8:12 - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<RefgenSpec> {
        GainW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn vreflo_sel(&mut self) -> VrefloSelW<RefgenSpec> {
        VrefloSelW::new(self, 16)
    }
    #[doc = "Bit 23 - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    #[must_use]
    pub fn vreflo_int(&mut self) -> VrefloIntW<RefgenSpec> {
        VrefloIntW::new(self, 23)
    }
}
#[doc = "Reference Generator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`refgen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refgen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefgenSpec;
impl crate::RegisterSpec for RefgenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refgen::R`](R) reader structure"]
impl crate::Readable for RefgenSpec {}
#[doc = "`write(|w| ..)` method takes [`refgen::W`](W) writer structure"]
impl crate::Writable for RefgenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REFGEN to value 0"]
impl crate::Resettable for RefgenSpec {
    const RESET_VALUE: u32 = 0;
}
