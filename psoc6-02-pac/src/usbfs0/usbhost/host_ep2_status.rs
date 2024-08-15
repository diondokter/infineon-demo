#[doc = "Register `HOST_EP2_STATUS` reader"]
pub type R = crate::R<HostEp2StatusSpec>;
#[doc = "Field `SIZE2` reader - These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP2 has finished. The indication range is from 0x000 to 0x40. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
pub type Size2R = crate::FieldReader;
#[doc = "Field `VAL_DATA` reader - This bit shows that there is valid data in the EP2 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
pub type ValDataR = crate::BitReader;
#[doc = "Field `INI_ST` reader - This bit shows that EP2 is initialized. If the BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' and EP2 is initialized, this bit is to '1'. '0' : Not Initialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately evne if BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '0' or '1'."]
pub type IniStR = crate::BitReader;
#[doc = "Field `RSVD_18` reader - N/A"]
pub type Rsvd18R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP2 has finished. The indication range is from 0x000 to 0x40. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub fn size2(&self) -> Size2R {
        Size2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - This bit shows that there is valid data in the EP2 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub fn val_data(&self) -> ValDataR {
        ValDataR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit shows that EP2 is initialized. If the BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' and EP2 is initialized, this bit is to '1'. '0' : Not Initialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately evne if BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '0' or '1'."]
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
#[doc = "Host Endpoint 2 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep2_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostEp2StatusSpec;
impl crate::RegisterSpec for HostEp2StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep2_status::R`](R) reader structure"]
impl crate::Readable for HostEp2StatusSpec {}
#[doc = "`reset()` method sets HOST_EP2_STATUS to value 0x0006_0000"]
impl crate::Resettable for HostEp2StatusSpec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
