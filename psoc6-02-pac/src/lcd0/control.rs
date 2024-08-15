#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `LS_EN` reader - Low speed (LS) generator enable 1: enable 0: disable"]
pub type LsEnR = crate::BitReader;
#[doc = "Field `LS_EN` writer - Low speed (LS) generator enable 1: enable 0: disable"]
pub type LsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_EN` reader - High speed (HS) generator enable 1: enable 0: disable"]
pub type HsEnR = crate::BitReader;
#[doc = "Field `HS_EN` writer - High speed (HS) generator enable 1: enable 0: disable"]
pub type HsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "HS/LS Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcdMode {
    #[doc = "0: Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    Ls = 0,
    #[doc = "1: Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    Hs = 1,
}
impl From<LcdMode> for bool {
    #[inline(always)]
    fn from(variant: LcdMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCD_MODE` reader - HS/LS Mode selection"]
pub type LcdModeR = crate::BitReader<LcdMode>;
impl LcdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcdMode {
        match self.bits {
            false => LcdMode::Ls,
            true => LcdMode::Hs,
        }
    }
    #[doc = "Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == LcdMode::Ls
    }
    #[doc = "Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == LcdMode::Hs
    }
}
#[doc = "Field `LCD_MODE` writer - HS/LS Mode selection"]
pub type LcdModeW<'a, REG> = crate::BitWriter<'a, REG, LcdMode>;
impl<'a, REG> LcdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut crate::W<REG> {
        self.variant(LcdMode::Ls)
    }
    #[doc = "Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(LcdMode::Hs)
    }
}
#[doc = "LCD driving waveform type configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    #[doc = "0: Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    TypeA = 0,
    #[doc = "1: Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    TypeB = 1,
}
impl From<Type> for bool {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE` reader - LCD driving waveform type configuration."]
pub type TypeR = crate::BitReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Type {
        match self.bits {
            false => Type::TypeA,
            true => Type::TypeB,
        }
    }
    #[doc = "Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    #[inline(always)]
    pub fn is_type_a(&self) -> bool {
        *self == Type::TypeA
    }
    #[doc = "Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    #[inline(always)]
    pub fn is_type_b(&self) -> bool {
        *self == Type::TypeB
    }
}
#[doc = "Field `TYPE` writer - LCD driving waveform type configuration."]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG, Type>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    #[inline(always)]
    pub fn type_a(self) -> &'a mut crate::W<REG> {
        self.variant(Type::TypeA)
    }
    #[doc = "Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    #[inline(always)]
    pub fn type_b(self) -> &'a mut crate::W<REG> {
        self.variant(Type::TypeB)
    }
}
#[doc = "Driving mode configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpMode {
    #[doc = "0: PWM Mode"]
    Pwm = 0,
    #[doc = "1: Digital Correlation Mode"]
    Correlation = 1,
}
impl From<OpMode> for bool {
    #[inline(always)]
    fn from(variant: OpMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OP_MODE` reader - Driving mode configuration"]
pub type OpModeR = crate::BitReader<OpMode>;
impl OpModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OpMode {
        match self.bits {
            false => OpMode::Pwm,
            true => OpMode::Correlation,
        }
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == OpMode::Pwm
    }
    #[doc = "Digital Correlation Mode"]
    #[inline(always)]
    pub fn is_correlation(&self) -> bool {
        *self == OpMode::Correlation
    }
}
#[doc = "Field `OP_MODE` writer - Driving mode configuration"]
pub type OpModeW<'a, REG> = crate::BitWriter<'a, REG, OpMode>;
impl<'a, REG> OpModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(OpMode::Pwm)
    }
    #[doc = "Digital Correlation Mode"]
    #[inline(always)]
    pub fn correlation(self) -> &'a mut crate::W<REG> {
        self.variant(OpMode::Correlation)
    }
}
#[doc = "PWM bias selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bias {
    #[doc = "0: 1/2 Bias"]
    Half = 0,
    #[doc = "1: 1/3 Bias"]
    Third = 1,
    #[doc = "2: 1/4 Bias (not supported by LS generator)"]
    Fourth = 2,
    #[doc = "3: 1/5 Bias (not supported by LS generator)"]
    Fifth = 3,
}
impl From<Bias> for u8 {
    #[inline(always)]
    fn from(variant: Bias) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bias {
    type Ux = u8;
}
impl crate::IsEnum for Bias {}
#[doc = "Field `BIAS` reader - PWM bias selection"]
pub type BiasR = crate::FieldReader<Bias>;
impl BiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bias {
        match self.bits {
            0 => Bias::Half,
            1 => Bias::Third,
            2 => Bias::Fourth,
            3 => Bias::Fifth,
            _ => unreachable!(),
        }
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Bias::Half
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn is_third(&self) -> bool {
        *self == Bias::Third
    }
    #[doc = "1/4 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == Bias::Fourth
    }
    #[doc = "1/5 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn is_fifth(&self) -> bool {
        *self == Bias::Fifth
    }
}
#[doc = "Field `BIAS` writer - PWM bias selection"]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bias, crate::Safe>;
impl<'a, REG> BiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Half)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn third(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Third)
    }
    #[doc = "1/4 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Fourth)
    }
    #[doc = "1/5 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn fifth(self) -> &'a mut crate::W<REG> {
        self.variant(Bias::Fifth)
    }
}
#[doc = "Field `COM_NUM` reader - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
pub type ComNumR = crate::FieldReader;
#[doc = "Field `COM_NUM` writer - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
pub type ComNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LS_EN_STAT` reader - LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
pub type LsEnStatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn ls_en(&self) -> LsEnR {
        LsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn hs_en(&self) -> HsEnR {
        HsEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HS/LS Mode selection"]
    #[inline(always)]
    pub fn lcd_mode(&self) -> LcdModeR {
        LcdModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD driving waveform type configuration."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Driving mode configuration"]
    #[inline(always)]
    pub fn op_mode(&self) -> OpModeR {
        OpModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - PWM bias selection"]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:11 - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub fn com_num(&self) -> ComNumR {
        ComNumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
    #[inline(always)]
    pub fn ls_en_stat(&self) -> LsEnStatR {
        LsEnStatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn ls_en(&mut self) -> LsEnW<ControlSpec> {
        LsEnW::new(self, 0)
    }
    #[doc = "Bit 1 - High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_en(&mut self) -> HsEnW<ControlSpec> {
        HsEnW::new(self, 1)
    }
    #[doc = "Bit 2 - HS/LS Mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_mode(&mut self) -> LcdModeW<ControlSpec> {
        LcdModeW::new(self, 2)
    }
    #[doc = "Bit 3 - LCD driving waveform type configuration."]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<ControlSpec> {
        TypeW::new(self, 3)
    }
    #[doc = "Bit 4 - Driving mode configuration"]
    #[inline(always)]
    #[must_use]
    pub fn op_mode(&mut self) -> OpModeW<ControlSpec> {
        OpModeW::new(self, 4)
    }
    #[doc = "Bits 5:6 - PWM bias selection"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BiasW<ControlSpec> {
        BiasW::new(self, 5)
    }
    #[doc = "Bits 8:11 - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    #[must_use]
    pub fn com_num(&mut self) -> ComNumW<ControlSpec> {
        ComNumW::new(self, 8)
    }
}
#[doc = "LCD Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
