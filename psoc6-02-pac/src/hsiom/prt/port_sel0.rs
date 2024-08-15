#[doc = "Register `PORT_SEL0` reader"]
pub type R = crate::R<PortSel0Spec>;
#[doc = "Register `PORT_SEL0` writer"]
pub type W = crate::W<PortSel0Spec>;
#[doc = "Selects connection for IO pin 0 route.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Io0Sel {
    #[doc = "0: GPIO controls 'out'"]
    Gpio = 0,
    #[doc = "1: GPIO controls 'out', DSI controls 'output enable'"]
    GpioDsi = 1,
    #[doc = "2: DSI controls 'out' and 'output enable'"]
    DsiDsi = 2,
    #[doc = "3: DSI controls 'out', GPIO controls 'output enable'"]
    DsiGpio = 3,
    #[doc = "4: Analog mux bus A"]
    Amuxa = 4,
    #[doc = "5: Analog mux bus B"]
    Amuxb = 5,
    #[doc = "6: Analog mux bus A, DSI control"]
    AmuxaDsi = 6,
    #[doc = "7: Analog mux bus B, DSI control"]
    AmuxbDsi = 7,
    #[doc = "8: Active functionality 0"]
    Act0 = 8,
    #[doc = "9: Active functionality 1"]
    Act1 = 9,
    #[doc = "10: Active functionality 2"]
    Act2 = 10,
    #[doc = "11: Active functionality 3"]
    Act3 = 11,
    #[doc = "12: DeepSleep functionality 0"]
    Ds0 = 12,
    #[doc = "13: DeepSleep functionality 1"]
    Ds1 = 13,
    #[doc = "14: DeepSleep functionality 2"]
    Ds2 = 14,
    #[doc = "15: DeepSleep functionality 3"]
    Ds3 = 15,
    #[doc = "16: Active functionality 4"]
    Act4 = 16,
    #[doc = "17: Active functionality 5"]
    Act5 = 17,
    #[doc = "18: Active functionality 6"]
    Act6 = 18,
    #[doc = "19: Active functionality 7"]
    Act7 = 19,
    #[doc = "20: Active functionality 8"]
    Act8 = 20,
    #[doc = "21: Active functionality 9"]
    Act9 = 21,
    #[doc = "22: Active functionality 10"]
    Act10 = 22,
    #[doc = "23: Active functionality 11"]
    Act11 = 23,
    #[doc = "24: Active functionality 12"]
    Act12 = 24,
    #[doc = "25: Active functionality 13"]
    Act13 = 25,
    #[doc = "26: Active functionality 14"]
    Act14 = 26,
    #[doc = "27: Active functionality 15"]
    Act15 = 27,
    #[doc = "28: DeepSleep functionality 4"]
    Ds4 = 28,
    #[doc = "29: DeepSleep functionality 5"]
    Ds5 = 29,
    #[doc = "30: DeepSleep functionality 6"]
    Ds6 = 30,
    #[doc = "31: DeepSleep functionality 7"]
    Ds7 = 31,
}
impl From<Io0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Io0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Io0Sel {
    type Ux = u8;
}
impl crate::IsEnum for Io0Sel {}
#[doc = "Field `IO0_SEL` reader - Selects connection for IO pin 0 route."]
pub type Io0SelR = crate::FieldReader<Io0Sel>;
impl Io0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io0Sel {
        match self.bits {
            0 => Io0Sel::Gpio,
            1 => Io0Sel::GpioDsi,
            2 => Io0Sel::DsiDsi,
            3 => Io0Sel::DsiGpio,
            4 => Io0Sel::Amuxa,
            5 => Io0Sel::Amuxb,
            6 => Io0Sel::AmuxaDsi,
            7 => Io0Sel::AmuxbDsi,
            8 => Io0Sel::Act0,
            9 => Io0Sel::Act1,
            10 => Io0Sel::Act2,
            11 => Io0Sel::Act3,
            12 => Io0Sel::Ds0,
            13 => Io0Sel::Ds1,
            14 => Io0Sel::Ds2,
            15 => Io0Sel::Ds3,
            16 => Io0Sel::Act4,
            17 => Io0Sel::Act5,
            18 => Io0Sel::Act6,
            19 => Io0Sel::Act7,
            20 => Io0Sel::Act8,
            21 => Io0Sel::Act9,
            22 => Io0Sel::Act10,
            23 => Io0Sel::Act11,
            24 => Io0Sel::Act12,
            25 => Io0Sel::Act13,
            26 => Io0Sel::Act14,
            27 => Io0Sel::Act15,
            28 => Io0Sel::Ds4,
            29 => Io0Sel::Ds5,
            30 => Io0Sel::Ds6,
            31 => Io0Sel::Ds7,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO controls 'out'"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Io0Sel::Gpio
    }
    #[doc = "GPIO controls 'out', DSI controls 'output enable'"]
    #[inline(always)]
    pub fn is_gpio_dsi(&self) -> bool {
        *self == Io0Sel::GpioDsi
    }
    #[doc = "DSI controls 'out' and 'output enable'"]
    #[inline(always)]
    pub fn is_dsi_dsi(&self) -> bool {
        *self == Io0Sel::DsiDsi
    }
    #[doc = "DSI controls 'out', GPIO controls 'output enable'"]
    #[inline(always)]
    pub fn is_dsi_gpio(&self) -> bool {
        *self == Io0Sel::DsiGpio
    }
    #[doc = "Analog mux bus A"]
    #[inline(always)]
    pub fn is_amuxa(&self) -> bool {
        *self == Io0Sel::Amuxa
    }
    #[doc = "Analog mux bus B"]
    #[inline(always)]
    pub fn is_amuxb(&self) -> bool {
        *self == Io0Sel::Amuxb
    }
    #[doc = "Analog mux bus A, DSI control"]
    #[inline(always)]
    pub fn is_amuxa_dsi(&self) -> bool {
        *self == Io0Sel::AmuxaDsi
    }
    #[doc = "Analog mux bus B, DSI control"]
    #[inline(always)]
    pub fn is_amuxb_dsi(&self) -> bool {
        *self == Io0Sel::AmuxbDsi
    }
    #[doc = "Active functionality 0"]
    #[inline(always)]
    pub fn is_act_0(&self) -> bool {
        *self == Io0Sel::Act0
    }
    #[doc = "Active functionality 1"]
    #[inline(always)]
    pub fn is_act_1(&self) -> bool {
        *self == Io0Sel::Act1
    }
    #[doc = "Active functionality 2"]
    #[inline(always)]
    pub fn is_act_2(&self) -> bool {
        *self == Io0Sel::Act2
    }
    #[doc = "Active functionality 3"]
    #[inline(always)]
    pub fn is_act_3(&self) -> bool {
        *self == Io0Sel::Act3
    }
    #[doc = "DeepSleep functionality 0"]
    #[inline(always)]
    pub fn is_ds_0(&self) -> bool {
        *self == Io0Sel::Ds0
    }
    #[doc = "DeepSleep functionality 1"]
    #[inline(always)]
    pub fn is_ds_1(&self) -> bool {
        *self == Io0Sel::Ds1
    }
    #[doc = "DeepSleep functionality 2"]
    #[inline(always)]
    pub fn is_ds_2(&self) -> bool {
        *self == Io0Sel::Ds2
    }
    #[doc = "DeepSleep functionality 3"]
    #[inline(always)]
    pub fn is_ds_3(&self) -> bool {
        *self == Io0Sel::Ds3
    }
    #[doc = "Active functionality 4"]
    #[inline(always)]
    pub fn is_act_4(&self) -> bool {
        *self == Io0Sel::Act4
    }
    #[doc = "Active functionality 5"]
    #[inline(always)]
    pub fn is_act_5(&self) -> bool {
        *self == Io0Sel::Act5
    }
    #[doc = "Active functionality 6"]
    #[inline(always)]
    pub fn is_act_6(&self) -> bool {
        *self == Io0Sel::Act6
    }
    #[doc = "Active functionality 7"]
    #[inline(always)]
    pub fn is_act_7(&self) -> bool {
        *self == Io0Sel::Act7
    }
    #[doc = "Active functionality 8"]
    #[inline(always)]
    pub fn is_act_8(&self) -> bool {
        *self == Io0Sel::Act8
    }
    #[doc = "Active functionality 9"]
    #[inline(always)]
    pub fn is_act_9(&self) -> bool {
        *self == Io0Sel::Act9
    }
    #[doc = "Active functionality 10"]
    #[inline(always)]
    pub fn is_act_10(&self) -> bool {
        *self == Io0Sel::Act10
    }
    #[doc = "Active functionality 11"]
    #[inline(always)]
    pub fn is_act_11(&self) -> bool {
        *self == Io0Sel::Act11
    }
    #[doc = "Active functionality 12"]
    #[inline(always)]
    pub fn is_act_12(&self) -> bool {
        *self == Io0Sel::Act12
    }
    #[doc = "Active functionality 13"]
    #[inline(always)]
    pub fn is_act_13(&self) -> bool {
        *self == Io0Sel::Act13
    }
    #[doc = "Active functionality 14"]
    #[inline(always)]
    pub fn is_act_14(&self) -> bool {
        *self == Io0Sel::Act14
    }
    #[doc = "Active functionality 15"]
    #[inline(always)]
    pub fn is_act_15(&self) -> bool {
        *self == Io0Sel::Act15
    }
    #[doc = "DeepSleep functionality 4"]
    #[inline(always)]
    pub fn is_ds_4(&self) -> bool {
        *self == Io0Sel::Ds4
    }
    #[doc = "DeepSleep functionality 5"]
    #[inline(always)]
    pub fn is_ds_5(&self) -> bool {
        *self == Io0Sel::Ds5
    }
    #[doc = "DeepSleep functionality 6"]
    #[inline(always)]
    pub fn is_ds_6(&self) -> bool {
        *self == Io0Sel::Ds6
    }
    #[doc = "DeepSleep functionality 7"]
    #[inline(always)]
    pub fn is_ds_7(&self) -> bool {
        *self == Io0Sel::Ds7
    }
}
#[doc = "Field `IO0_SEL` writer - Selects connection for IO pin 0 route."]
pub type Io0SelW<'a, REG> = crate::FieldWriter<'a, REG, 5, Io0Sel, crate::Safe>;
impl<'a, REG> Io0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO controls 'out'"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Gpio)
    }
    #[doc = "GPIO controls 'out', DSI controls 'output enable'"]
    #[inline(always)]
    pub fn gpio_dsi(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::GpioDsi)
    }
    #[doc = "DSI controls 'out' and 'output enable'"]
    #[inline(always)]
    pub fn dsi_dsi(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::DsiDsi)
    }
    #[doc = "DSI controls 'out', GPIO controls 'output enable'"]
    #[inline(always)]
    pub fn dsi_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::DsiGpio)
    }
    #[doc = "Analog mux bus A"]
    #[inline(always)]
    pub fn amuxa(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Amuxa)
    }
    #[doc = "Analog mux bus B"]
    #[inline(always)]
    pub fn amuxb(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Amuxb)
    }
    #[doc = "Analog mux bus A, DSI control"]
    #[inline(always)]
    pub fn amuxa_dsi(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::AmuxaDsi)
    }
    #[doc = "Analog mux bus B, DSI control"]
    #[inline(always)]
    pub fn amuxb_dsi(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::AmuxbDsi)
    }
    #[doc = "Active functionality 0"]
    #[inline(always)]
    pub fn act_0(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act0)
    }
    #[doc = "Active functionality 1"]
    #[inline(always)]
    pub fn act_1(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act1)
    }
    #[doc = "Active functionality 2"]
    #[inline(always)]
    pub fn act_2(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act2)
    }
    #[doc = "Active functionality 3"]
    #[inline(always)]
    pub fn act_3(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act3)
    }
    #[doc = "DeepSleep functionality 0"]
    #[inline(always)]
    pub fn ds_0(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds0)
    }
    #[doc = "DeepSleep functionality 1"]
    #[inline(always)]
    pub fn ds_1(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds1)
    }
    #[doc = "DeepSleep functionality 2"]
    #[inline(always)]
    pub fn ds_2(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds2)
    }
    #[doc = "DeepSleep functionality 3"]
    #[inline(always)]
    pub fn ds_3(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds3)
    }
    #[doc = "Active functionality 4"]
    #[inline(always)]
    pub fn act_4(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act4)
    }
    #[doc = "Active functionality 5"]
    #[inline(always)]
    pub fn act_5(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act5)
    }
    #[doc = "Active functionality 6"]
    #[inline(always)]
    pub fn act_6(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act6)
    }
    #[doc = "Active functionality 7"]
    #[inline(always)]
    pub fn act_7(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act7)
    }
    #[doc = "Active functionality 8"]
    #[inline(always)]
    pub fn act_8(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act8)
    }
    #[doc = "Active functionality 9"]
    #[inline(always)]
    pub fn act_9(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act9)
    }
    #[doc = "Active functionality 10"]
    #[inline(always)]
    pub fn act_10(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act10)
    }
    #[doc = "Active functionality 11"]
    #[inline(always)]
    pub fn act_11(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act11)
    }
    #[doc = "Active functionality 12"]
    #[inline(always)]
    pub fn act_12(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act12)
    }
    #[doc = "Active functionality 13"]
    #[inline(always)]
    pub fn act_13(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act13)
    }
    #[doc = "Active functionality 14"]
    #[inline(always)]
    pub fn act_14(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act14)
    }
    #[doc = "Active functionality 15"]
    #[inline(always)]
    pub fn act_15(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Act15)
    }
    #[doc = "DeepSleep functionality 4"]
    #[inline(always)]
    pub fn ds_4(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds4)
    }
    #[doc = "DeepSleep functionality 5"]
    #[inline(always)]
    pub fn ds_5(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds5)
    }
    #[doc = "DeepSleep functionality 6"]
    #[inline(always)]
    pub fn ds_6(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds6)
    }
    #[doc = "DeepSleep functionality 7"]
    #[inline(always)]
    pub fn ds_7(self) -> &'a mut crate::W<REG> {
        self.variant(Io0Sel::Ds7)
    }
}
#[doc = "Field `IO1_SEL` reader - Selects connection for IO pin 1 route."]
pub type Io1SelR = crate::FieldReader;
#[doc = "Field `IO1_SEL` writer - Selects connection for IO pin 1 route."]
pub type Io1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO2_SEL` reader - Selects connection for IO pin 2 route."]
pub type Io2SelR = crate::FieldReader;
#[doc = "Field `IO2_SEL` writer - Selects connection for IO pin 2 route."]
pub type Io2SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO3_SEL` reader - Selects connection for IO pin 3 route."]
pub type Io3SelR = crate::FieldReader;
#[doc = "Field `IO3_SEL` writer - Selects connection for IO pin 3 route."]
pub type Io3SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Selects connection for IO pin 0 route."]
    #[inline(always)]
    pub fn io0_sel(&self) -> Io0SelR {
        Io0SelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 1 route."]
    #[inline(always)]
    pub fn io1_sel(&self) -> Io1SelR {
        Io1SelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 2 route."]
    #[inline(always)]
    pub fn io2_sel(&self) -> Io2SelR {
        Io2SelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 3 route."]
    #[inline(always)]
    pub fn io3_sel(&self) -> Io3SelR {
        Io3SelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects connection for IO pin 0 route."]
    #[inline(always)]
    #[must_use]
    pub fn io0_sel(&mut self) -> Io0SelW<PortSel0Spec> {
        Io0SelW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 1 route."]
    #[inline(always)]
    #[must_use]
    pub fn io1_sel(&mut self) -> Io1SelW<PortSel0Spec> {
        Io1SelW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 2 route."]
    #[inline(always)]
    #[must_use]
    pub fn io2_sel(&mut self) -> Io2SelW<PortSel0Spec> {
        Io2SelW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 3 route."]
    #[inline(always)]
    #[must_use]
    pub fn io3_sel(&mut self) -> Io3SelW<PortSel0Spec> {
        Io3SelW::new(self, 24)
    }
}
#[doc = "Port selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`port_sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortSel0Spec;
impl crate::RegisterSpec for PortSel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_sel0::R`](R) reader structure"]
impl crate::Readable for PortSel0Spec {}
#[doc = "`write(|w| ..)` method takes [`port_sel0::W`](W) writer structure"]
impl crate::Writable for PortSel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PORT_SEL0 to value 0"]
impl crate::Resettable for PortSel0Spec {
    const RESET_VALUE: u32 = 0;
}
