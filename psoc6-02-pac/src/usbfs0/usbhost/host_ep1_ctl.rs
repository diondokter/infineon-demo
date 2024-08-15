#[doc = "Register `HOST_EP1_CTL` reader"]
pub type R = crate::R<HostEp1CtlSpec>;
#[doc = "Register `HOST_EP1_CTL` writer"]
pub type W = crate::W<HostEp1CtlSpec>;
#[doc = "Field `PKS1` reader - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
pub type Pks1R = crate::FieldReader<u16>;
#[doc = "Field `PKS1` writer - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
pub type Pks1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NULLE` reader - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
pub type NulleR = crate::BitReader;
#[doc = "Field `NULLE` writer - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
pub type NulleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAE` reader - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
pub type DmaeR = crate::BitReader;
#[doc = "Field `DMAE` writer - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFINI` reader - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
pub type BfiniR = crate::BitReader;
#[doc = "Field `BFINI` writer - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
pub type BfiniW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
    #[inline(always)]
    pub fn pks1(&self) -> Pks1R {
        Pks1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 10 - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub fn nulle(&self) -> NulleR {
        NulleR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
    #[inline(always)]
    pub fn bfini(&self) -> BfiniR {
        BfiniR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
    #[inline(always)]
    #[must_use]
    pub fn pks1(&mut self) -> Pks1W<HostEp1CtlSpec> {
        Pks1W::new(self, 0)
    }
    #[doc = "Bit 10 - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    #[must_use]
    pub fn nulle(&mut self) -> NulleW<HostEp1CtlSpec> {
        NulleW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DmaeW<HostEp1CtlSpec> {
        DmaeW::new(self, 11)
    }
    #[doc = "Bit 12 - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<HostEp1CtlSpec> {
        DirW::new(self, 12)
    }
    #[doc = "Bit 15 - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
    #[inline(always)]
    #[must_use]
    pub fn bfini(&mut self) -> BfiniW<HostEp1CtlSpec> {
        BfiniW::new(self, 15)
    }
}
#[doc = "Host Endpoint 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep1_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostEp1CtlSpec;
impl crate::RegisterSpec for HostEp1CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep1_ctl::R`](R) reader structure"]
impl crate::Readable for HostEp1CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ep1_ctl::W`](W) writer structure"]
impl crate::Writable for HostEp1CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_EP1_CTL to value 0x8100"]
impl crate::Resettable for HostEp1CtlSpec {
    const RESET_VALUE: u32 = 0x8100;
}
