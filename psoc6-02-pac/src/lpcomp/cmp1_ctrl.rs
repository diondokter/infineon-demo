#[doc = "Register `CMP1_CTRL` reader"]
pub type R = crate::R<Cmp1CtrlSpec>;
#[doc = "Register `CMP1_CTRL` writer"]
pub type W = crate::W<Cmp1CtrlSpec>;
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode1 {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    Ulp = 1,
    #[doc = "2: Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    Lp = 2,
    #[doc = "3: Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    Normal = 3,
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(variant: Mode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode1 {
    type Ux = u8;
}
impl crate::IsEnum for Mode1 {}
#[doc = "Field `MODE1` reader - Operating mode for the comparator"]
pub type Mode1R = crate::FieldReader<Mode1>;
impl Mode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode1 {
        match self.bits {
            0 => Mode1::Off,
            1 => Mode1::Ulp,
            2 => Mode1::Lp,
            3 => Mode1::Normal,
            _ => unreachable!(),
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode1::Off
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Mode1::Ulp
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Mode1::Lp
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Mode1::Normal
    }
}
#[doc = "Field `MODE1` writer - Operating mode for the comparator"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode1, crate::Safe>;
impl<'a, REG> Mode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Off)
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Ulp)
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Lp)
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Normal)
    }
}
#[doc = "Field `HYST1` reader - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type Hyst1R = crate::BitReader;
#[doc = "Field `HYST1` writer - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type Hyst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inttype1 {
    #[doc = "0: Disabled, no interrupts will be detected"]
    Disable = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Both rising and falling edges"]
    Both = 3,
}
impl From<Inttype1> for u8 {
    #[inline(always)]
    fn from(variant: Inttype1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inttype1 {
    type Ux = u8;
}
impl crate::IsEnum for Inttype1 {}
#[doc = "Field `INTTYPE1` reader - Sets which edge will trigger an IRQ"]
pub type Inttype1R = crate::FieldReader<Inttype1>;
impl Inttype1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inttype1 {
        match self.bits {
            0 => Inttype1::Disable,
            1 => Inttype1::Rising,
            2 => Inttype1::Falling,
            3 => Inttype1::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inttype1::Disable
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Inttype1::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Inttype1::Falling
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Inttype1::Both
    }
}
#[doc = "Field `INTTYPE1` writer - Sets which edge will trigger an IRQ"]
pub type Inttype1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Inttype1, crate::Safe>;
impl<'a, REG> Inttype1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype1::Disable)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype1::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype1::Falling)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype1::Both)
    }
}
#[doc = "Field `DSI_BYPASS1` reader - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DsiBypass1R = crate::BitReader;
#[doc = "Field `DSI_BYPASS1` writer - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DsiBypass1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_LEVEL1` reader - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DsiLevel1R = crate::BitReader;
#[doc = "Field `DSI_LEVEL1` writer - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DsiLevel1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst1(&self) -> Hyst1R {
        Hyst1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype1(&self) -> Inttype1R {
        Inttype1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass1(&self) -> DsiBypass1R {
        DsiBypass1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level1(&self) -> DsiLevel1R {
        DsiLevel1R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<Cmp1CtrlSpec> {
        Mode1W::new(self, 0)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst1(&mut self) -> Hyst1W<Cmp1CtrlSpec> {
        Hyst1W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn inttype1(&mut self) -> Inttype1W<Cmp1CtrlSpec> {
        Inttype1W::new(self, 6)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_bypass1(&mut self) -> DsiBypass1W<Cmp1CtrlSpec> {
        DsiBypass1W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_level1(&mut self) -> DsiLevel1W<Cmp1CtrlSpec> {
        DsiLevel1W::new(self, 11)
    }
}
#[doc = "Comparator 1 control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1CtrlSpec;
impl crate::RegisterSpec for Cmp1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1_ctrl::R`](R) reader structure"]
impl crate::Readable for Cmp1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1_ctrl::W`](W) writer structure"]
impl crate::Writable for Cmp1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP1_CTRL to value 0"]
impl crate::Resettable for Cmp1CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
