#[doc = "Register `MUX_SWITCH_DS_CTRL` reader"]
pub type R = crate::R<MuxSwitchDsCtrlSpec>;
#[doc = "Register `MUX_SWITCH_DS_CTRL` writer"]
pub type W = crate::W<MuxSwitchDsCtrlSpec>;
#[doc = "Field `MUX_DS_CTRL_P0` reader - for P0 switches"]
pub type MuxDsCtrlP0R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P0` writer - for P0 switches"]
pub type MuxDsCtrlP0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_P1` reader - for P1 switches"]
pub type MuxDsCtrlP1R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P1` writer - for P1 switches"]
pub type MuxDsCtrlP1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_P2` reader - for P2 switches"]
pub type MuxDsCtrlP2R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P2` writer - for P2 switches"]
pub type MuxDsCtrlP2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_P3` reader - for P3 switches"]
pub type MuxDsCtrlP3R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P3` writer - for P3 switches"]
pub type MuxDsCtrlP3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_P4` reader - for P4 switches"]
pub type MuxDsCtrlP4R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P4` writer - for P4 switches"]
pub type MuxDsCtrlP4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_P5` reader - for P5 switches"]
pub type MuxDsCtrlP5R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P5` writer - for P5 switches"]
pub type MuxDsCtrlP5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_P6` reader - for P6 switches"]
pub type MuxDsCtrlP6R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P6` writer - for P6 switches"]
pub type MuxDsCtrlP6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_P7` reader - for P7 switches"]
pub type MuxDsCtrlP7R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_P7` writer - for P7 switches"]
pub type MuxDsCtrlP7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_VSSA` reader - for vssa switch"]
pub type MuxDsCtrlVssaR = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_VSSA` writer - for vssa switch"]
pub type MuxDsCtrlVssaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_TEMP` reader - for temp switch"]
pub type MuxDsCtrlTempR = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_TEMP` writer - for temp switch"]
pub type MuxDsCtrlTempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_AMUXBUSA` reader - for amuxbusa switch"]
pub type MuxDsCtrlAmuxbusaR = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_AMUXBUSA` writer - for amuxbusa switch"]
pub type MuxDsCtrlAmuxbusaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_AMUXBUSB` reader - for amuxbusb switches"]
pub type MuxDsCtrlAmuxbusbR = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_AMUXBUSB` writer - for amuxbusb switches"]
pub type MuxDsCtrlAmuxbusbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_SARBUS0` reader - for sarbus0 switch"]
pub type MuxDsCtrlSarbus0R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_SARBUS0` writer - for sarbus0 switch"]
pub type MuxDsCtrlSarbus0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_DS_CTRL_SARBUS1` reader - for sarbus1 switch"]
pub type MuxDsCtrlSarbus1R = crate::BitReader;
#[doc = "Field `MUX_DS_CTRL_SARBUS1` writer - for sarbus1 switch"]
pub type MuxDsCtrlSarbus1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p0(&self) -> MuxDsCtrlP0R {
        MuxDsCtrlP0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p1(&self) -> MuxDsCtrlP1R {
        MuxDsCtrlP1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p2(&self) -> MuxDsCtrlP2R {
        MuxDsCtrlP2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p3(&self) -> MuxDsCtrlP3R {
        MuxDsCtrlP3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p4(&self) -> MuxDsCtrlP4R {
        MuxDsCtrlP4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p5(&self) -> MuxDsCtrlP5R {
        MuxDsCtrlP5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p6(&self) -> MuxDsCtrlP6R {
        MuxDsCtrlP6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p7(&self) -> MuxDsCtrlP7R {
        MuxDsCtrlP7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_vssa(&self) -> MuxDsCtrlVssaR {
        MuxDsCtrlVssaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_temp(&self) -> MuxDsCtrlTempR {
        MuxDsCtrlTempR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_amuxbusa(&self) -> MuxDsCtrlAmuxbusaR {
        MuxDsCtrlAmuxbusaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_amuxbusb(&self) -> MuxDsCtrlAmuxbusbR {
        MuxDsCtrlAmuxbusbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_sarbus0(&self) -> MuxDsCtrlSarbus0R {
        MuxDsCtrlSarbus0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_sarbus1(&self) -> MuxDsCtrlSarbus1R {
        MuxDsCtrlSarbus1R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p0(&mut self) -> MuxDsCtrlP0W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP0W::new(self, 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p1(&mut self) -> MuxDsCtrlP1W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP1W::new(self, 1)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p2(&mut self) -> MuxDsCtrlP2W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP2W::new(self, 2)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p3(&mut self) -> MuxDsCtrlP3W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP3W::new(self, 3)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p4(&mut self) -> MuxDsCtrlP4W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP4W::new(self, 4)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p5(&mut self) -> MuxDsCtrlP5W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP5W::new(self, 5)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p6(&mut self) -> MuxDsCtrlP6W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP6W::new(self, 6)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_p7(&mut self) -> MuxDsCtrlP7W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlP7W::new(self, 7)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_vssa(&mut self) -> MuxDsCtrlVssaW<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlVssaW::new(self, 16)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_temp(&mut self) -> MuxDsCtrlTempW<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlTempW::new(self, 17)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_amuxbusa(&mut self) -> MuxDsCtrlAmuxbusaW<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlAmuxbusaW::new(self, 18)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_amuxbusb(&mut self) -> MuxDsCtrlAmuxbusbW<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlAmuxbusbW::new(self, 19)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_sarbus0(&mut self) -> MuxDsCtrlSarbus0W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlSarbus0W::new(self, 22)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_ds_ctrl_sarbus1(&mut self) -> MuxDsCtrlSarbus1W<MuxSwitchDsCtrlSpec> {
        MuxDsCtrlSarbus1W::new(self, 23)
    }
}
#[doc = "SARMUX switch DSI control\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch_ds_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux_switch_ds_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxSwitchDsCtrlSpec;
impl crate::RegisterSpec for MuxSwitchDsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux_switch_ds_ctrl::R`](R) reader structure"]
impl crate::Readable for MuxSwitchDsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mux_switch_ds_ctrl::W`](W) writer structure"]
impl crate::Writable for MuxSwitchDsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUX_SWITCH_DS_CTRL to value 0"]
impl crate::Resettable for MuxSwitchDsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
