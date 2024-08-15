#[doc = "Register `RX_MATCH` reader"]
pub type R = crate::R<RxMatchSpec>;
#[doc = "Register `RX_MATCH` writer"]
pub type W = crate::W<RxMatchSpec>;
#[doc = "Field `ADDR` reader - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASK` reader - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<RxMatchSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<RxMatchSpec> {
        MaskW::new(self, 16)
    }
}
#[doc = "Slave address and mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_match::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_match::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxMatchSpec;
impl crate::RegisterSpec for RxMatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_match::R`](R) reader structure"]
impl crate::Readable for RxMatchSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_match::W`](W) writer structure"]
impl crate::Writable for RxMatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_MATCH to value 0"]
impl crate::Resettable for RxMatchSpec {
    const RESET_VALUE: u32 = 0;
}
