#[doc = "Register `WDT_CTL` reader"]
pub type R = crate::R<WdtCtlSpec>;
#[doc = "Register `WDT_CTL` writer"]
pub type W = crate::W<WdtCtlSpec>;
#[doc = "Field `WDT_EN` reader - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
pub type WdtEnR = crate::BitReader;
#[doc = "Field `WDT_EN` writer - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
pub type WdtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WdtLock {
    #[doc = "0: No effect"]
    NoChg = 0,
    #[doc = "1: Clears bit 0"]
    Clr0 = 1,
    #[doc = "2: Clears bit 1"]
    Clr1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    Set01 = 3,
}
impl From<WdtLock> for u8 {
    #[inline(always)]
    fn from(variant: WdtLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WdtLock {
    type Ux = u8;
}
impl crate::IsEnum for WdtLock {}
#[doc = "Field `WDT_LOCK` reader - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
pub type WdtLockR = crate::FieldReader<WdtLock>;
impl WdtLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtLock {
        match self.bits {
            0 => WdtLock::NoChg,
            1 => WdtLock::Clr0,
            2 => WdtLock::Clr1,
            3 => WdtLock::Set01,
            _ => unreachable!(),
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == WdtLock::NoChg
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == WdtLock::Clr0
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == WdtLock::Clr1
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == WdtLock::Set01
    }
}
#[doc = "Field `WDT_LOCK` writer - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
pub type WdtLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, WdtLock, crate::Safe>;
impl<'a, REG> WdtLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut crate::W<REG> {
        self.variant(WdtLock::NoChg)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut crate::W<REG> {
        self.variant(WdtLock::Clr0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut crate::W<REG> {
        self.variant(WdtLock::Clr1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut crate::W<REG> {
        self.variant(WdtLock::Set01)
    }
}
impl R {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WdtEnR {
        WdtEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub fn wdt_lock(&self) -> WdtLockR {
        WdtLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WdtEnW<WdtCtlSpec> {
        WdtEnW::new(self, 0)
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_lock(&mut self) -> WdtLockW<WdtCtlSpec> {
        WdtLockW::new(self, 30)
    }
}
#[doc = "Watchdog Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtCtlSpec;
impl crate::RegisterSpec for WdtCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_ctl::R`](R) reader structure"]
impl crate::Readable for WdtCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_ctl::W`](W) writer structure"]
impl crate::Writable for WdtCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT_CTL to value 0xc000_0001"]
impl crate::Resettable for WdtCtlSpec {
    const RESET_VALUE: u32 = 0xc000_0001;
}
