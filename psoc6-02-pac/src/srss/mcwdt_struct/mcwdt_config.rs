#[doc = "Register `MCWDT_CONFIG` reader"]
pub type R = crate::R<McwdtConfigSpec>;
#[doc = "Register `MCWDT_CONFIG` writer"]
pub type W = crate::W<McwdtConfigSpec>;
#[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WdtMode0 {
    #[doc = "0: Do nothing"]
    Nothing = 0,
    #[doc = "1: Assert WDT_INTx"]
    Int = 1,
    #[doc = "2: Assert WDT Reset"]
    Reset = 2,
    #[doc = "3: Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    IntThenReset = 3,
}
impl From<WdtMode0> for u8 {
    #[inline(always)]
    fn from(variant: WdtMode0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WdtMode0 {
    type Ux = u8;
}
impl crate::IsEnum for WdtMode0 {}
#[doc = "Field `WDT_MODE0` reader - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
pub type WdtMode0R = crate::FieldReader<WdtMode0>;
impl WdtMode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtMode0 {
        match self.bits {
            0 => WdtMode0::Nothing,
            1 => WdtMode0::Int,
            2 => WdtMode0::Reset,
            3 => WdtMode0::IntThenReset,
            _ => unreachable!(),
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == WdtMode0::Nothing
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WdtMode0::Int
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WdtMode0::Reset
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn is_int_then_reset(&self) -> bool {
        *self == WdtMode0::IntThenReset
    }
}
#[doc = "Field `WDT_MODE0` writer - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
pub type WdtMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2, WdtMode0, crate::Safe>;
impl<'a, REG> WdtMode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode0::Nothing)
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode0::Int)
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode0::Reset)
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn int_then_reset(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode0::IntThenReset)
    }
}
#[doc = "Field `WDT_CLEAR0` reader - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
pub type WdtClear0R = crate::BitReader;
#[doc = "Field `WDT_CLEAR0` writer - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
pub type WdtClear0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CASCADE0_1` reader - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
pub type WdtCascade0_1R = crate::BitReader;
#[doc = "Field `WDT_CASCADE0_1` writer - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
pub type WdtCascade0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WdtMode1 {
    #[doc = "0: Do nothing"]
    Nothing = 0,
    #[doc = "1: Assert WDT_INTx"]
    Int = 1,
    #[doc = "2: Assert WDT Reset"]
    Reset = 2,
    #[doc = "3: Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    IntThenReset = 3,
}
impl From<WdtMode1> for u8 {
    #[inline(always)]
    fn from(variant: WdtMode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WdtMode1 {
    type Ux = u8;
}
impl crate::IsEnum for WdtMode1 {}
#[doc = "Field `WDT_MODE1` reader - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
pub type WdtMode1R = crate::FieldReader<WdtMode1>;
impl WdtMode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtMode1 {
        match self.bits {
            0 => WdtMode1::Nothing,
            1 => WdtMode1::Int,
            2 => WdtMode1::Reset,
            3 => WdtMode1::IntThenReset,
            _ => unreachable!(),
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == WdtMode1::Nothing
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WdtMode1::Int
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WdtMode1::Reset
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn is_int_then_reset(&self) -> bool {
        *self == WdtMode1::IntThenReset
    }
}
#[doc = "Field `WDT_MODE1` writer - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
pub type WdtMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2, WdtMode1, crate::Safe>;
impl<'a, REG> WdtMode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode1::Nothing)
    }
    #[doc = "Assert WDT_INTx"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode1::Int)
    }
    #[doc = "Assert WDT Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode1::Reset)
    }
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    #[inline(always)]
    pub fn int_then_reset(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode1::IntThenReset)
    }
}
#[doc = "Field `WDT_CLEAR1` reader - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
pub type WdtClear1R = crate::BitReader;
#[doc = "Field `WDT_CLEAR1` writer - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
pub type WdtClear1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CASCADE1_2` reader - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
pub type WdtCascade1_2R = crate::BitReader;
#[doc = "Field `WDT_CASCADE1_2` writer - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
pub type WdtCascade1_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog Counter 2 Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtMode2 {
    #[doc = "0: Free running counter with no interrupt requests"]
    Nothing = 0,
    #[doc = "1: Free running counter with interrupt request that occurs one LFCLK cycle after the specified bit in CTR2 toggles (see WDT_BITS2)."]
    Int = 1,
}
impl From<WdtMode2> for bool {
    #[inline(always)]
    fn from(variant: WdtMode2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_MODE2` reader - Watchdog Counter 2 Mode."]
pub type WdtMode2R = crate::BitReader<WdtMode2>;
impl WdtMode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtMode2 {
        match self.bits {
            false => WdtMode2::Nothing,
            true => WdtMode2::Int,
        }
    }
    #[doc = "Free running counter with no interrupt requests"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == WdtMode2::Nothing
    }
    #[doc = "Free running counter with interrupt request that occurs one LFCLK cycle after the specified bit in CTR2 toggles (see WDT_BITS2)."]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WdtMode2::Int
    }
}
#[doc = "Field `WDT_MODE2` writer - Watchdog Counter 2 Mode."]
pub type WdtMode2W<'a, REG> = crate::BitWriter<'a, REG, WdtMode2>;
impl<'a, REG> WdtMode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free running counter with no interrupt requests"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode2::Nothing)
    }
    #[doc = "Free running counter with interrupt request that occurs one LFCLK cycle after the specified bit in CTR2 toggles (see WDT_BITS2)."]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(WdtMode2::Int)
    }
}
#[doc = "Field `WDT_BITS2` reader - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
pub type WdtBits2R = crate::FieldReader;
#[doc = "Field `WDT_BITS2` writer - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
pub type WdtBits2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    pub fn wdt_mode0(&self) -> WdtMode0R {
        WdtMode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
    #[inline(always)]
    pub fn wdt_clear0(&self) -> WdtClear0R {
        WdtClear0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    pub fn wdt_cascade0_1(&self) -> WdtCascade0_1R {
        WdtCascade0_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
    #[inline(always)]
    pub fn wdt_mode1(&self) -> WdtMode1R {
        WdtMode1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
    #[inline(always)]
    pub fn wdt_clear1(&self) -> WdtClear1R {
        WdtClear1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
    #[inline(always)]
    pub fn wdt_cascade1_2(&self) -> WdtCascade1_2R {
        WdtCascade1_2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Watchdog Counter 2 Mode."]
    #[inline(always)]
    pub fn wdt_mode2(&self) -> WdtMode2R {
        WdtMode2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    pub fn wdt_bits2(&self) -> WdtBits2R {
        WdtBits2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_mode0(&mut self) -> WdtMode0W<McwdtConfigSpec> {
        WdtMode0W::new(self, 0)
    }
    #[doc = "Bit 2 - Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_clear0(&mut self) -> WdtClear0W<McwdtConfigSpec> {
        WdtClear0W::new(self, 2)
    }
    #[doc = "Bit 3 - Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cascade0_1(&mut self) -> WdtCascade0_1W<McwdtConfigSpec> {
        WdtCascade0_1W::new(self, 3)
    }
    #[doc = "Bits 8:9 - Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_mode1(&mut self) -> WdtMode1W<McwdtConfigSpec> {
        WdtMode1W::new(self, 8)
    }
    #[doc = "Bit 10 - Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_clear1(&mut self) -> WdtClear1W<McwdtConfigSpec> {
        WdtClear1W::new(self, 10)
    }
    #[doc = "Bit 11 - Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cascade1_2(&mut self) -> WdtCascade1_2W<McwdtConfigSpec> {
        WdtCascade1_2W::new(self, 11)
    }
    #[doc = "Bit 16 - Watchdog Counter 2 Mode."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_mode2(&mut self) -> WdtMode2W<McwdtConfigSpec> {
        WdtMode2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_bits2(&mut self) -> WdtBits2W<McwdtConfigSpec> {
        WdtBits2W::new(self, 24)
    }
}
#[doc = "Multi-Counter Watchdog Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtConfigSpec;
impl crate::RegisterSpec for McwdtConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_config::R`](R) reader structure"]
impl crate::Readable for McwdtConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_config::W`](W) writer structure"]
impl crate::Writable for McwdtConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_CONFIG to value 0"]
impl crate::Resettable for McwdtConfigSpec {
    const RESET_VALUE: u32 = 0;
}
