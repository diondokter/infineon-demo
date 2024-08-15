#[doc = "Register `INTR_HOST_EP` reader"]
pub type R = crate::R<IntrHostEpSpec>;
#[doc = "Register `INTR_HOST_EP` writer"]
pub type W = crate::W<IntrHostEpSpec>;
#[doc = "Field `EP1DRQ` reader - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type Ep1drqR = crate::BitReader;
#[doc = "Field `EP1DRQ` writer - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type Ep1drqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1SPK` reader - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The EP1SPK bit is not set during data transfer in the OUT direction."]
pub type Ep1spkR = crate::BitReader;
#[doc = "Field `EP1SPK` writer - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The EP1SPK bit is not set during data transfer in the OUT direction."]
pub type Ep1spkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2DRQ` reader - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If packet transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type Ep2drqR = crate::BitReader;
#[doc = "Field `EP2DRQ` writer - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If packet transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type Ep2drqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2SPK` reader - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS1 in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
pub type Ep2spkR = crate::BitReader;
#[doc = "Field `EP2SPK` writer - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS1 in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
pub type Ep2spkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep1drq(&self) -> Ep1drqR {
        Ep1drqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The EP1SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep1spk(&self) -> Ep1spkR {
        Ep1spkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If packet transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep2drq(&self) -> Ep2drqR {
        Ep2drqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS1 in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep2spk(&self) -> Ep2spkR {
        Ep2spkR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    #[must_use]
    pub fn ep1drq(&mut self) -> Ep1drqW<IntrHostEpSpec> {
        Ep1drqW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The EP1SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep1spk(&mut self) -> Ep1spkW<IntrHostEpSpec> {
        Ep1spkW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If packet transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    #[must_use]
    pub fn ep2drq(&mut self) -> Ep2drqW<IntrHostEpSpec> {
        Ep2drqW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS1 in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep2spk(&mut self) -> Ep2spkW<IntrHostEpSpec> {
        Ep2spkW::new(self, 5)
    }
}
#[doc = "Interrupt USB Host Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_host_ep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrHostEpSpec;
impl crate::RegisterSpec for IntrHostEpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep::R`](R) reader structure"]
impl crate::Readable for IntrHostEpSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_host_ep::W`](W) writer structure"]
impl crate::Writable for IntrHostEpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_HOST_EP to value 0"]
impl crate::Resettable for IntrHostEpSpec {
    const RESET_VALUE: u32 = 0;
}
