#[doc = "Register `HOST_ADDR` reader"]
pub type R = crate::R<HostAddrSpec>;
#[doc = "Register `HOST_ADDR` writer"]
pub type W = crate::W<HostAddrSpec>;
#[doc = "Field `ADDRESS` reader - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type AddressR = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<HostAddrSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Host Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostAddrSpec;
impl crate::RegisterSpec for HostAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_addr::R`](R) reader structure"]
impl crate::Readable for HostAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`host_addr::W`](W) writer structure"]
impl crate::Writable for HostAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_ADDR to value 0"]
impl crate::Resettable for HostAddrSpec {
    const RESET_VALUE: u32 = 0;
}
