#[doc = "Register `ADDR0` reader"]
pub type R = crate::R<Addr0Spec>;
#[doc = "Register `ADDR0` writer"]
pub type W = crate::W<Addr0Spec>;
#[doc = "Field `SUBREGION_DISABLE` reader - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (ATT0.REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by ATT0 applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
pub type SubregionDisableR = crate::FieldReader;
#[doc = "Field `SUBREGION_DISABLE` writer - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (ATT0.REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by ATT0 applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
pub type SubregionDisableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR24` reader - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT0.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
pub type Addr24R = crate::FieldReader<u32>;
#[doc = "Field `ADDR24` writer - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT0.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
pub type Addr24W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (ATT0.REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by ATT0 applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SubregionDisableR {
        SubregionDisableR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT0.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
    #[inline(always)]
    pub fn addr24(&self) -> Addr24R {
        Addr24R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (ATT0.REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by ATT0 applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    #[must_use]
    pub fn subregion_disable(&mut self) -> SubregionDisableW<Addr0Spec> {
        SubregionDisableW::new(self, 0)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT0.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn addr24(&mut self) -> Addr24W<Addr0Spec> {
        Addr24W::new(self, 8)
    }
}
#[doc = "SMPU region address 0 (slave structure)\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr0Spec;
impl crate::RegisterSpec for Addr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr0::R`](R) reader structure"]
impl crate::Readable for Addr0Spec {}
#[doc = "`write(|w| ..)` method takes [`addr0::W`](W) writer structure"]
impl crate::Writable for Addr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::Resettable for Addr0Spec {
    const RESET_VALUE: u32 = 0;
}
