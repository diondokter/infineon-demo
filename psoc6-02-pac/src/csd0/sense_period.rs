#[doc = "Register `SENSE_PERIOD` reader"]
pub type R = crate::R<SensePeriodSpec>;
#[doc = "Register `SENSE_PERIOD` writer"]
pub type W = crate::W<SensePeriodSpec>;
#[doc = "Field `SENSE_DIV` reader - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
pub type SenseDivR = crate::FieldReader<u16>;
#[doc = "Field `SENSE_DIV` writer - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
pub type SenseDivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LfsrSize {
    #[doc = "0: Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    Off = 0,
    #[doc = "1: 6-bit LFSR (G(x)=X^6 +X^4+X^3+ X+1, period= 63)"]
    _6b = 1,
    #[doc = "2: 7-bit LFSR (G(x)=X^7 +X^4+X^3+X^2+1, period= 127)"]
    _7b = 2,
    #[doc = "3: 9-bit LFSR (G(x)=X^9 +X^4+X^3+ X+1, period= 511)"]
    _9b = 3,
    #[doc = "4: 10-bit LFSR (G(x)=X^10+X^4+X^3+ X+1, period= 1023)"]
    _10b = 4,
    #[doc = "5: 8-bit LFSR (G(x)=X^8+X^4+X^3+X^2+1, period= 255)"]
    _8b = 5,
    #[doc = "6: 12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    _12b = 6,
}
impl From<LfsrSize> for u8 {
    #[inline(always)]
    fn from(variant: LfsrSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LfsrSize {
    type Ux = u8;
}
impl crate::IsEnum for LfsrSize {}
#[doc = "Field `LFSR_SIZE` reader - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
pub type LfsrSizeR = crate::FieldReader<LfsrSize>;
impl LfsrSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LfsrSize> {
        match self.bits {
            0 => Some(LfsrSize::Off),
            1 => Some(LfsrSize::_6b),
            2 => Some(LfsrSize::_7b),
            3 => Some(LfsrSize::_9b),
            4 => Some(LfsrSize::_10b),
            5 => Some(LfsrSize::_8b),
            6 => Some(LfsrSize::_12b),
            _ => None,
        }
    }
    #[doc = "Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LfsrSize::Off
    }
    #[doc = "6-bit LFSR (G(x)=X^6 +X^4+X^3+ X+1, period= 63)"]
    #[inline(always)]
    pub fn is_6b(&self) -> bool {
        *self == LfsrSize::_6b
    }
    #[doc = "7-bit LFSR (G(x)=X^7 +X^4+X^3+X^2+1, period= 127)"]
    #[inline(always)]
    pub fn is_7b(&self) -> bool {
        *self == LfsrSize::_7b
    }
    #[doc = "9-bit LFSR (G(x)=X^9 +X^4+X^3+ X+1, period= 511)"]
    #[inline(always)]
    pub fn is_9b(&self) -> bool {
        *self == LfsrSize::_9b
    }
    #[doc = "10-bit LFSR (G(x)=X^10+X^4+X^3+ X+1, period= 1023)"]
    #[inline(always)]
    pub fn is_10b(&self) -> bool {
        *self == LfsrSize::_10b
    }
    #[doc = "8-bit LFSR (G(x)=X^8+X^4+X^3+X^2+1, period= 255)"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == LfsrSize::_8b
    }
    #[doc = "12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    #[inline(always)]
    pub fn is_12b(&self) -> bool {
        *self == LfsrSize::_12b
    }
}
#[doc = "Field `LFSR_SIZE` writer - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
pub type LfsrSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, LfsrSize>;
impl<'a, REG> LfsrSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrSize::Off)
    }
    #[doc = "6-bit LFSR (G(x)=X^6 +X^4+X^3+ X+1, period= 63)"]
    #[inline(always)]
    pub fn _6b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrSize::_6b)
    }
    #[doc = "7-bit LFSR (G(x)=X^7 +X^4+X^3+X^2+1, period= 127)"]
    #[inline(always)]
    pub fn _7b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrSize::_7b)
    }
    #[doc = "9-bit LFSR (G(x)=X^9 +X^4+X^3+ X+1, period= 511)"]
    #[inline(always)]
    pub fn _9b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrSize::_9b)
    }
    #[doc = "10-bit LFSR (G(x)=X^10+X^4+X^3+ X+1, period= 1023)"]
    #[inline(always)]
    pub fn _10b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrSize::_10b)
    }
    #[doc = "8-bit LFSR (G(x)=X^8+X^4+X^3+X^2+1, period= 255)"]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrSize::_8b)
    }
    #[doc = "12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    #[inline(always)]
    pub fn _12b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrSize::_12b)
    }
}
#[doc = "Field `LFSR_SCALE` reader - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT&lt;&lt;LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
pub type LfsrScaleR = crate::FieldReader;
#[doc = "Field `LFSR_SCALE` writer - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT&lt;&lt;LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
pub type LfsrScaleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LFSR_CLEAR` reader - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
pub type LfsrClearR = crate::BitReader;
#[doc = "Field `LFSR_CLEAR` writer - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
pub type LfsrClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_LFSR_MSB` reader - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
pub type SelLfsrMsbR = crate::BitReader;
#[doc = "Field `SEL_LFSR_MSB` writer - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
pub type SelLfsrMsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LfsrBits {
    #[doc = "0: use 2 bits: range = \\[-2,1\\]"]
    _2b = 0,
    #[doc = "1: use 3 bits: range = \\[-4,3\\]"]
    _3b = 1,
    #[doc = "2: use 4 bits: range = \\[-8,7\\]"]
    _4b = 2,
    #[doc = "3: use 5 bits: range = \\[-16,15\\]
(default)"]
    _5b = 3,
}
impl From<LfsrBits> for u8 {
    #[inline(always)]
    fn from(variant: LfsrBits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LfsrBits {
    type Ux = u8;
}
impl crate::IsEnum for LfsrBits {}
#[doc = "Field `LFSR_BITS` reader - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
pub type LfsrBitsR = crate::FieldReader<LfsrBits>;
impl LfsrBitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LfsrBits {
        match self.bits {
            0 => LfsrBits::_2b,
            1 => LfsrBits::_3b,
            2 => LfsrBits::_4b,
            3 => LfsrBits::_5b,
            _ => unreachable!(),
        }
    }
    #[doc = "use 2 bits: range = \\[-2,1\\]"]
    #[inline(always)]
    pub fn is_2b(&self) -> bool {
        *self == LfsrBits::_2b
    }
    #[doc = "use 3 bits: range = \\[-4,3\\]"]
    #[inline(always)]
    pub fn is_3b(&self) -> bool {
        *self == LfsrBits::_3b
    }
    #[doc = "use 4 bits: range = \\[-8,7\\]"]
    #[inline(always)]
    pub fn is_4b(&self) -> bool {
        *self == LfsrBits::_4b
    }
    #[doc = "use 5 bits: range = \\[-16,15\\]
(default)"]
    #[inline(always)]
    pub fn is_5b(&self) -> bool {
        *self == LfsrBits::_5b
    }
}
#[doc = "Field `LFSR_BITS` writer - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
pub type LfsrBitsW<'a, REG> = crate::FieldWriter<'a, REG, 2, LfsrBits, crate::Safe>;
impl<'a, REG> LfsrBitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "use 2 bits: range = \\[-2,1\\]"]
    #[inline(always)]
    pub fn _2b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrBits::_2b)
    }
    #[doc = "use 3 bits: range = \\[-4,3\\]"]
    #[inline(always)]
    pub fn _3b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrBits::_3b)
    }
    #[doc = "use 4 bits: range = \\[-8,7\\]"]
    #[inline(always)]
    pub fn _4b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrBits::_4b)
    }
    #[doc = "use 5 bits: range = \\[-16,15\\]
(default)"]
    #[inline(always)]
    pub fn _5b(self) -> &'a mut crate::W<REG> {
        self.variant(LfsrBits::_5b)
    }
}
impl R {
    #[doc = "Bits 0:11 - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
    #[inline(always)]
    pub fn sense_div(&self) -> SenseDivR {
        SenseDivR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
    #[inline(always)]
    pub fn lfsr_size(&self) -> LfsrSizeR {
        LfsrSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT&lt;&lt;LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub fn lfsr_scale(&self) -> LfsrScaleR {
        LfsrScaleR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub fn lfsr_clear(&self) -> LfsrClearR {
        LfsrClearR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    pub fn sel_lfsr_msb(&self) -> SelLfsrMsbR {
        SelLfsrMsbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    pub fn lfsr_bits(&self) -> LfsrBitsR {
        LfsrBitsR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
    #[inline(always)]
    #[must_use]
    pub fn sense_div(&mut self) -> SenseDivW<SensePeriodSpec> {
        SenseDivW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_size(&mut self) -> LfsrSizeW<SensePeriodSpec> {
        LfsrSizeW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT&lt;&lt;LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_scale(&mut self) -> LfsrScaleW<SensePeriodSpec> {
        LfsrScaleW::new(self, 20)
    }
    #[doc = "Bit 24 - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_clear(&mut self) -> LfsrClearW<SensePeriodSpec> {
        LfsrClearW::new(self, 24)
    }
    #[doc = "Bit 25 - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn sel_lfsr_msb(&mut self) -> SelLfsrMsbW<SensePeriodSpec> {
        SelLfsrMsbW::new(self, 25)
    }
    #[doc = "Bits 26:27 - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_bits(&mut self) -> LfsrBitsW<SensePeriodSpec> {
        LfsrBitsW::new(self, 26)
    }
}
#[doc = "Sense clock period\n\nYou can [`read`](crate::Reg::read) this register and get [`sense_period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sense_period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SensePeriodSpec;
impl crate::RegisterSpec for SensePeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sense_period::R`](R) reader structure"]
impl crate::Readable for SensePeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`sense_period::W`](W) writer structure"]
impl crate::Writable for SensePeriodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SENSE_PERIOD to value 0x0c00_0000"]
impl crate::Resettable for SensePeriodSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
