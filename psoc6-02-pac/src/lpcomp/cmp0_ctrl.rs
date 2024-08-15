#[doc = "Register `CMP0_CTRL` reader"]
pub type R = crate::R<Cmp0CtrlSpec>;
#[doc = "Register `CMP0_CTRL` writer"]
pub type W = crate::W<Cmp0CtrlSpec>;
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode0 {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    Ulp = 1,
    #[doc = "2: Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    Lp = 2,
    #[doc = "3: Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    Normal = 3,
}
impl From<Mode0> for u8 {
    #[inline(always)]
    fn from(variant: Mode0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode0 {
    type Ux = u8;
}
impl crate::IsEnum for Mode0 {}
#[doc = "Field `MODE0` reader - Operating mode for the comparator"]
pub type Mode0R = crate::FieldReader<Mode0>;
impl Mode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode0 {
        match self.bits {
            0 => Mode0::Off,
            1 => Mode0::Ulp,
            2 => Mode0::Lp,
            3 => Mode0::Normal,
            _ => unreachable!(),
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode0::Off
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Mode0::Ulp
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Mode0::Lp
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Mode0::Normal
    }
}
#[doc = "Field `MODE0` writer - Operating mode for the comparator"]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode0, crate::Safe>;
impl<'a, REG> Mode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Off)
    }
    #[doc = "Ultra lowpower operating mode (uses less power, &lt; 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Ulp)
    }
    #[doc = "Low Power operating mode (uses more power, &lt;10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Lp)
    }
    #[doc = "Normal, full speed power operating mode (uses &lt;150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Normal)
    }
}
#[doc = "Field `HYST0` reader - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type Hyst0R = crate::BitReader;
#[doc = "Field `HYST0` writer - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub type Hyst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inttype0 {
    #[doc = "0: Disabled, no interrupts will be detected"]
    Disable = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Both rising and falling edges"]
    Both = 3,
}
impl From<Inttype0> for u8 {
    #[inline(always)]
    fn from(variant: Inttype0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inttype0 {
    type Ux = u8;
}
impl crate::IsEnum for Inttype0 {}
#[doc = "Field `INTTYPE0` reader - Sets which edge will trigger an IRQ"]
pub type Inttype0R = crate::FieldReader<Inttype0>;
impl Inttype0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inttype0 {
        match self.bits {
            0 => Inttype0::Disable,
            1 => Inttype0::Rising,
            2 => Inttype0::Falling,
            3 => Inttype0::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inttype0::Disable
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Inttype0::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Inttype0::Falling
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Inttype0::Both
    }
}
#[doc = "Field `INTTYPE0` writer - Sets which edge will trigger an IRQ"]
pub type Inttype0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Inttype0, crate::Safe>;
impl<'a, REG> Inttype0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype0::Disable)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype0::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype0::Falling)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype0::Both)
    }
}
#[doc = "Field `DSI_BYPASS0` reader - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DsiBypass0R = crate::BitReader;
#[doc = "Field `DSI_BYPASS0` writer - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DsiBypass0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_LEVEL0` reader - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DsiLevel0R = crate::BitReader;
#[doc = "Field `DSI_LEVEL0` writer - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub type DsiLevel0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst0(&self) -> Hyst0R {
        Hyst0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype0(&self) -> Inttype0R {
        Inttype0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass0(&self) -> DsiBypass0R {
        DsiBypass0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level0(&self) -> DsiLevel0R {
        DsiLevel0R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> Mode0W<Cmp0CtrlSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst0(&mut self) -> Hyst0W<Cmp0CtrlSpec> {
        Hyst0W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn inttype0(&mut self) -> Inttype0W<Cmp0CtrlSpec> {
        Inttype0W::new(self, 6)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_bypass0(&mut self) -> DsiBypass0W<Cmp0CtrlSpec> {
        DsiBypass0W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_level0(&mut self) -> DsiLevel0W<Cmp0CtrlSpec> {
        DsiLevel0W::new(self, 11)
    }
}
#[doc = "Comparator 0 control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp0CtrlSpec;
impl crate::RegisterSpec for Cmp0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp0_ctrl::R`](R) reader structure"]
impl crate::Readable for Cmp0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp0_ctrl::W`](W) writer structure"]
impl crate::Writable for Cmp0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP0_CTRL to value 0"]
impl crate::Resettable for Cmp0CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
