#[doc = "Register `HOST_EP1_STATUS` reader"]
pub type R = crate::R<HostEp1StatusSpec>;
#[doc = "Field `SIZE1` reader - These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP1 has finished. The indication range is from 0x000 to 0x100. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
pub type Size1R = crate::FieldReader<u16>;
#[doc = "Field `VAL_DATA` reader - This bit shows that there is valid data in the EP1 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
pub type ValDataR = crate::BitReader;
#[doc = "Field `INI_ST` reader - This bit shows that EP1 is initialized. If the init bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' and EP1 is initialized, this bit is to '1'. '0' : Not initiatialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately even if BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '0' or '1'. Read this bit to confirm the transition."]
pub type IniStR = crate::BitReader;
#[doc = "Field `RSVD_18` reader - N/A"]
pub type Rsvd18R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP1 has finished. The indication range is from 0x000 to 0x100. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub fn size1(&self) -> Size1R {
        Size1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - This bit shows that there is valid data in the EP1 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub fn val_data(&self) -> ValDataR {
        ValDataR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit shows that EP1 is initialized. If the init bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' and EP1 is initialized, this bit is to '1'. '0' : Not initiatialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately even if BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '0' or '1'. Read this bit to confirm the transition."]
    #[inline(always)]
    pub fn ini_st(&self) -> IniStR {
        IniStR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn rsvd_18(&self) -> Rsvd18R {
        Rsvd18R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Host Endpoint 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostEp1StatusSpec;
impl crate::RegisterSpec for HostEp1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep1_status::R`](R) reader structure"]
impl crate::Readable for HostEp1StatusSpec {}
#[doc = "`reset()` method sets HOST_EP1_STATUS to value 0x0006_0000"]
impl crate::Resettable for HostEp1StatusSpec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
