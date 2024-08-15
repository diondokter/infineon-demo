#[doc = "Register `CHAN_CONFIG[%s]` reader"]
pub type R = crate::R<ChanConfigSpec>;
#[doc = "Register `CHAN_CONFIG[%s]` writer"]
pub type W = crate::W<ChanConfigSpec>;
#[doc = "Field `POS_PIN_ADDR` reader - Address of the pin to be sampled by this channel (connected to Vplus)"]
pub type PosPinAddrR = crate::FieldReader;
#[doc = "Field `POS_PIN_ADDR` writer - Address of the pin to be sampled by this channel (connected to Vplus)"]
pub type PosPinAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Address of the port that contains the pin to be sampled by this channel (connected to Vplus)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PosPortAddr {
    #[doc = "0: SARMUX pins."]
    Sarmux = 0,
    #[doc = "1: CTB0"]
    Ctb0 = 1,
    #[doc = "2: CTB1"]
    Ctb1 = 2,
    #[doc = "3: CTB2"]
    Ctb2 = 3,
    #[doc = "4: CTB3"]
    Ctb3 = 4,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    ArouteVirt2 = 5,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    ArouteVirt1 = 6,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SarmuxVirt = 7,
}
impl From<PosPortAddr> for u8 {
    #[inline(always)]
    fn from(variant: PosPortAddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PosPortAddr {
    type Ux = u8;
}
impl crate::IsEnum for PosPortAddr {}
#[doc = "Field `POS_PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
pub type PosPortAddrR = crate::FieldReader<PosPortAddr>;
impl PosPortAddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PosPortAddr {
        match self.bits {
            0 => PosPortAddr::Sarmux,
            1 => PosPortAddr::Ctb0,
            2 => PosPortAddr::Ctb1,
            3 => PosPortAddr::Ctb2,
            4 => PosPortAddr::Ctb3,
            5 => PosPortAddr::ArouteVirt2,
            6 => PosPortAddr::ArouteVirt1,
            7 => PosPortAddr::SarmuxVirt,
            _ => unreachable!(),
        }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == PosPortAddr::Sarmux
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        *self == PosPortAddr::Ctb0
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        *self == PosPortAddr::Ctb1
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        *self == PosPortAddr::Ctb2
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        *self == PosPortAddr::Ctb3
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        *self == PosPortAddr::ArouteVirt2
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        *self == PosPortAddr::ArouteVirt1
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == PosPortAddr::SarmuxVirt
    }
}
#[doc = "Field `POS_PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
pub type PosPortAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3, PosPortAddr, crate::Safe>;
impl<'a, REG> PosPortAddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::Sarmux)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::Ctb0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::Ctb1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::Ctb2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::Ctb3)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::ArouteVirt2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::ArouteVirt1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut crate::W<REG> {
        self.variant(PosPortAddr::SarmuxVirt)
    }
}
#[doc = "Field `DIFFERENTIAL_EN` reader - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
pub type DifferentialEnR = crate::BitReader;
#[doc = "Field `DIFFERENTIAL_EN` writer - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
pub type DifferentialEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type AvgEnR = crate::BitReader;
#[doc = "Field `AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type AvgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_TIME_SEL` reader - Sample time select: select which of the 4 global sample times to use for this channel"]
pub type SampleTimeSelR = crate::FieldReader;
#[doc = "Field `SAMPLE_TIME_SEL` writer - Sample time select: select which of the 4 global sample times to use for this channel"]
pub type SampleTimeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NEG_PIN_ADDR` reader - Address of the neg pin to be sampled by this channel."]
pub type NegPinAddrR = crate::FieldReader;
#[doc = "Field `NEG_PIN_ADDR` writer - Address of the neg pin to be sampled by this channel."]
pub type NegPinAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Address of the neg port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NegPortAddr {
    #[doc = "0: SARMUX pins."]
    Sarmux = 0,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    ArouteVirt2 = 5,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    ArouteVirt1 = 6,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SarmuxVirt = 7,
}
impl From<NegPortAddr> for u8 {
    #[inline(always)]
    fn from(variant: NegPortAddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NegPortAddr {
    type Ux = u8;
}
impl crate::IsEnum for NegPortAddr {}
#[doc = "Field `NEG_PORT_ADDR` reader - Address of the neg port that contains the pin to be sampled by this channel."]
pub type NegPortAddrR = crate::FieldReader<NegPortAddr>;
impl NegPortAddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NegPortAddr> {
        match self.bits {
            0 => Some(NegPortAddr::Sarmux),
            5 => Some(NegPortAddr::ArouteVirt2),
            6 => Some(NegPortAddr::ArouteVirt1),
            7 => Some(NegPortAddr::SarmuxVirt),
            _ => None,
        }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == NegPortAddr::Sarmux
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        *self == NegPortAddr::ArouteVirt2
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        *self == NegPortAddr::ArouteVirt1
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == NegPortAddr::SarmuxVirt
    }
}
#[doc = "Field `NEG_PORT_ADDR` writer - Address of the neg port that contains the pin to be sampled by this channel."]
pub type NegPortAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3, NegPortAddr>;
impl<'a, REG> NegPortAddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut crate::W<REG> {
        self.variant(NegPortAddr::Sarmux)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut crate::W<REG> {
        self.variant(NegPortAddr::ArouteVirt2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut crate::W<REG> {
        self.variant(NegPortAddr::ArouteVirt1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut crate::W<REG> {
        self.variant(NegPortAddr::SarmuxVirt)
    }
}
#[doc = "Field `NEG_ADDR_EN` reader - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
pub type NegAddrEnR = crate::BitReader;
#[doc = "Field `NEG_ADDR_EN` writer - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
pub type NegAddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_OUT_EN` reader - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub type DsiOutEnR = crate::BitReader;
#[doc = "Field `DSI_OUT_EN` writer - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub type DsiOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_pin_addr(&self) -> PosPinAddrR {
        PosPinAddrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_port_addr(&self) -> PosPortAddrR {
        PosPortAddrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn differential_en(&self) -> DifferentialEnR {
        DifferentialEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&self) -> AvgEnR {
        AvgEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&self) -> SampleTimeSelR {
        SampleTimeSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_pin_addr(&self) -> NegPinAddrR {
        NegPinAddrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_port_addr(&self) -> NegPortAddrR {
        NegPortAddrR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub fn neg_addr_en(&self) -> NegAddrEnR {
        NegAddrEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&self) -> DsiOutEnR {
        DsiOutEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    #[must_use]
    pub fn pos_pin_addr(&mut self) -> PosPinAddrW<ChanConfigSpec> {
        PosPinAddrW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    #[must_use]
    pub fn pos_port_addr(&mut self) -> PosPortAddrW<ChanConfigSpec> {
        PosPortAddrW::new(self, 4)
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    #[must_use]
    pub fn differential_en(&mut self) -> DifferentialEnW<ChanConfigSpec> {
        DifferentialEnW::new(self, 8)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    #[must_use]
    pub fn avg_en(&mut self) -> AvgEnW<ChanConfigSpec> {
        AvgEnW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time_sel(&mut self) -> SampleTimeSelW<ChanConfigSpec> {
        SampleTimeSelW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    #[must_use]
    pub fn neg_pin_addr(&mut self) -> NegPinAddrW<ChanConfigSpec> {
        NegPinAddrW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    #[must_use]
    pub fn neg_port_addr(&mut self) -> NegPortAddrW<ChanConfigSpec> {
        NegPortAddrW::new(self, 20)
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    #[must_use]
    pub fn neg_addr_en(&mut self) -> NegAddrEnW<ChanConfigSpec> {
        NegAddrEnW::new(self, 24)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_out_en(&mut self) -> DsiOutEnW<ChanConfigSpec> {
        DsiOutEnW::new(self, 31)
    }
}
#[doc = "Channel configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanConfigSpec;
impl crate::RegisterSpec for ChanConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_config::R`](R) reader structure"]
impl crate::Readable for ChanConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`chan_config::W`](W) writer structure"]
impl crate::Writable for ChanConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHAN_CONFIG[%s]
to value 0"]
impl crate::Resettable for ChanConfigSpec {
    const RESET_VALUE: u32 = 0;
}
