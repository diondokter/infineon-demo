#[doc = "Register `MUX_SWITCH_SQ_CTRL` reader"]
pub type R = crate::R<MuxSwitchSqCtrlSpec>;
#[doc = "Register `MUX_SWITCH_SQ_CTRL` writer"]
pub type W = crate::W<MuxSwitchSqCtrlSpec>;
#[doc = "Field `MUX_SQ_CTRL_P0` reader - for P0 switches"]
pub type MuxSqCtrlP0R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P0` writer - for P0 switches"]
pub type MuxSqCtrlP0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P1` reader - for P1 switches"]
pub type MuxSqCtrlP1R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P1` writer - for P1 switches"]
pub type MuxSqCtrlP1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P2` reader - for P2 switches"]
pub type MuxSqCtrlP2R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P2` writer - for P2 switches"]
pub type MuxSqCtrlP2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P3` reader - for P3 switches"]
pub type MuxSqCtrlP3R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P3` writer - for P3 switches"]
pub type MuxSqCtrlP3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P4` reader - for P4 switches"]
pub type MuxSqCtrlP4R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P4` writer - for P4 switches"]
pub type MuxSqCtrlP4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P5` reader - for P5 switches"]
pub type MuxSqCtrlP5R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P5` writer - for P5 switches"]
pub type MuxSqCtrlP5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P6` reader - for P6 switches"]
pub type MuxSqCtrlP6R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P6` writer - for P6 switches"]
pub type MuxSqCtrlP6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P7` reader - for P7 switches"]
pub type MuxSqCtrlP7R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P7` writer - for P7 switches"]
pub type MuxSqCtrlP7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_VSSA` reader - for vssa switch"]
pub type MuxSqCtrlVssaR = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_VSSA` writer - for vssa switch"]
pub type MuxSqCtrlVssaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_TEMP` reader - for temp switch"]
pub type MuxSqCtrlTempR = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_TEMP` writer - for temp switch"]
pub type MuxSqCtrlTempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSA` reader - for amuxbusa switch"]
pub type MuxSqCtrlAmuxbusaR = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSA` writer - for amuxbusa switch"]
pub type MuxSqCtrlAmuxbusaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSB` reader - for amuxbusb switches"]
pub type MuxSqCtrlAmuxbusbR = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSB` writer - for amuxbusb switches"]
pub type MuxSqCtrlAmuxbusbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_SARBUS0` reader - for sarbus0 switch"]
pub type MuxSqCtrlSarbus0R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_SARBUS0` writer - for sarbus0 switch"]
pub type MuxSqCtrlSarbus0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_SARBUS1` reader - for sarbus1 switch"]
pub type MuxSqCtrlSarbus1R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_SARBUS1` writer - for sarbus1 switch"]
pub type MuxSqCtrlSarbus1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p0(&self) -> MuxSqCtrlP0R {
        MuxSqCtrlP0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p1(&self) -> MuxSqCtrlP1R {
        MuxSqCtrlP1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p2(&self) -> MuxSqCtrlP2R {
        MuxSqCtrlP2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p3(&self) -> MuxSqCtrlP3R {
        MuxSqCtrlP3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p4(&self) -> MuxSqCtrlP4R {
        MuxSqCtrlP4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p5(&self) -> MuxSqCtrlP5R {
        MuxSqCtrlP5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p6(&self) -> MuxSqCtrlP6R {
        MuxSqCtrlP6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p7(&self) -> MuxSqCtrlP7R {
        MuxSqCtrlP7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_vssa(&self) -> MuxSqCtrlVssaR {
        MuxSqCtrlVssaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_temp(&self) -> MuxSqCtrlTempR {
        MuxSqCtrlTempR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusa(&self) -> MuxSqCtrlAmuxbusaR {
        MuxSqCtrlAmuxbusaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusb(&self) -> MuxSqCtrlAmuxbusbR {
        MuxSqCtrlAmuxbusbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus0(&self) -> MuxSqCtrlSarbus0R {
        MuxSqCtrlSarbus0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus1(&self) -> MuxSqCtrlSarbus1R {
        MuxSqCtrlSarbus1R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p0(&mut self) -> MuxSqCtrlP0W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP0W::new(self, 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p1(&mut self) -> MuxSqCtrlP1W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP1W::new(self, 1)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p2(&mut self) -> MuxSqCtrlP2W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP2W::new(self, 2)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p3(&mut self) -> MuxSqCtrlP3W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP3W::new(self, 3)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p4(&mut self) -> MuxSqCtrlP4W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP4W::new(self, 4)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p5(&mut self) -> MuxSqCtrlP5W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP5W::new(self, 5)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p6(&mut self) -> MuxSqCtrlP6W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP6W::new(self, 6)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p7(&mut self) -> MuxSqCtrlP7W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlP7W::new(self, 7)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_vssa(&mut self) -> MuxSqCtrlVssaW<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlVssaW::new(self, 16)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_temp(&mut self) -> MuxSqCtrlTempW<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlTempW::new(self, 17)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_amuxbusa(&mut self) -> MuxSqCtrlAmuxbusaW<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlAmuxbusaW::new(self, 18)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_amuxbusb(&mut self) -> MuxSqCtrlAmuxbusbW<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlAmuxbusbW::new(self, 19)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_sarbus0(&mut self) -> MuxSqCtrlSarbus0W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlSarbus0W::new(self, 22)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_sarbus1(&mut self) -> MuxSqCtrlSarbus1W<MuxSwitchSqCtrlSpec> {
        MuxSqCtrlSarbus1W::new(self, 23)
    }
}
#[doc = "SARMUX switch Sar Sequencer control\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch_sq_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux_switch_sq_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxSwitchSqCtrlSpec;
impl crate::RegisterSpec for MuxSwitchSqCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux_switch_sq_ctrl::R`](R) reader structure"]
impl crate::Readable for MuxSwitchSqCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mux_switch_sq_ctrl::W`](W) writer structure"]
impl crate::Writable for MuxSwitchSqCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUX_SWITCH_SQ_CTRL to value 0"]
impl crate::Resettable for MuxSwitchSqCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
