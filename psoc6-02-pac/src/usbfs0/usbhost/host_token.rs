#[doc = "Register `HOST_TOKEN` reader"]
pub type R = crate::R<HostTokenSpec>;
#[doc = "Register `HOST_TOKEN` writer"]
pub type W = crate::W<HostTokenSpec>;
#[doc = "Field `ENDPT` reader - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EndptR = crate::FieldReader;
#[doc = "Field `ENDPT` writer - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EndptW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "These bits send a token according to the current settings. After operation is complete, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Mode should be USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tknen {
    #[doc = "0: Sends no data."]
    None = 0,
    #[doc = "1: Sends SETUP token."]
    Setup = 1,
    #[doc = "2: Sends IN token."]
    In = 2,
    #[doc = "3: Sends OUT token."]
    Out = 3,
    #[doc = "4: Sends SOF token."]
    Sof = 4,
    #[doc = "5: Sends Isochronous IN."]
    IsoIn = 5,
    #[doc = "6: Sends Isochronous OUT."]
    IsoOut = 6,
    #[doc = "7: N/A"]
    Rsv = 7,
}
impl From<Tknen> for u8 {
    #[inline(always)]
    fn from(variant: Tknen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tknen {
    type Ux = u8;
}
impl crate::IsEnum for Tknen {}
#[doc = "Field `TKNEN` reader - These bits send a token according to the current settings. After operation is complete, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Mode should be USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type TknenR = crate::FieldReader<Tknen>;
impl TknenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tknen {
        match self.bits {
            0 => Tknen::None,
            1 => Tknen::Setup,
            2 => Tknen::In,
            3 => Tknen::Out,
            4 => Tknen::Sof,
            5 => Tknen::IsoIn,
            6 => Tknen::IsoOut,
            7 => Tknen::Rsv,
            _ => unreachable!(),
        }
    }
    #[doc = "Sends no data."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Tknen::None
    }
    #[doc = "Sends SETUP token."]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == Tknen::Setup
    }
    #[doc = "Sends IN token."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Tknen::In
    }
    #[doc = "Sends OUT token."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Tknen::Out
    }
    #[doc = "Sends SOF token."]
    #[inline(always)]
    pub fn is_sof(&self) -> bool {
        *self == Tknen::Sof
    }
    #[doc = "Sends Isochronous IN."]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == Tknen::IsoIn
    }
    #[doc = "Sends Isochronous OUT."]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == Tknen::IsoOut
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsv(&self) -> bool {
        *self == Tknen::Rsv
    }
}
#[doc = "Field `TKNEN` writer - These bits send a token according to the current settings. After operation is complete, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Mode should be USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type TknenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tknen, crate::Safe>;
impl<'a, REG> TknenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sends no data."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::None)
    }
    #[doc = "Sends SETUP token."]
    #[inline(always)]
    pub fn setup(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::Setup)
    }
    #[doc = "Sends IN token."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::In)
    }
    #[doc = "Sends OUT token."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::Out)
    }
    #[doc = "Sends SOF token."]
    #[inline(always)]
    pub fn sof(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::Sof)
    }
    #[doc = "Sends Isochronous IN."]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::IsoIn)
    }
    #[doc = "Sends Isochronous OUT."]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::IsoOut)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsv(self) -> &'a mut crate::W<REG> {
        self.variant(Tknen::Rsv)
    }
}
#[doc = "Field `TGGL` reader - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't reset to the default value even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
pub type TgglR = crate::BitReader;
#[doc = "Field `TGGL` writer - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't reset to the default value even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
pub type TgglW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn endpt(&self) -> EndptR {
        EndptR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - These bits send a token according to the current settings. After operation is complete, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Mode should be USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn tknen(&self) -> TknenR {
        TknenR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't reset to the default value even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
    #[inline(always)]
    pub fn tggl(&self) -> TgglR {
        TgglR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn endpt(&mut self) -> EndptW<HostTokenSpec> {
        EndptW::new(self, 0)
    }
    #[doc = "Bits 4:6 - These bits send a token according to the current settings. After operation is complete, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Mode should be USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tknen(&mut self) -> TknenW<HostTokenSpec> {
        TknenW::new(self, 4)
    }
    #[doc = "Bit 8 - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't reset to the default value even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
    #[inline(always)]
    #[must_use]
    pub fn tggl(&mut self) -> TgglW<HostTokenSpec> {
        TgglW::new(self, 8)
    }
}
#[doc = "Host Token Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_token::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_token::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostTokenSpec;
impl crate::RegisterSpec for HostTokenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_token::R`](R) reader structure"]
impl crate::Readable for HostTokenSpec {}
#[doc = "`write(|w| ..)` method takes [`host_token::W`](W) writer structure"]
impl crate::Writable for HostTokenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_TOKEN to value 0"]
impl crate::Resettable for HostTokenSpec {
    const RESET_VALUE: u32 = 0;
}
