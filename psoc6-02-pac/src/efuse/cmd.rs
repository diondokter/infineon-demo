#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `BIT_DATA` reader - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
pub type BitDataR = crate::BitReader;
#[doc = "Field `BIT_DATA` writer - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
pub type BitDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ADDR` reader - Bit address. This field specifies a bit within a Byte."]
pub type BitAddrR = crate::FieldReader;
#[doc = "Field `BIT_ADDR` writer - Bit address. This field specifies a bit within a Byte."]
pub type BitAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BYTE_ADDR` reader - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
pub type ByteAddrR = crate::FieldReader;
#[doc = "Field `BYTE_ADDR` writer - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
pub type ByteAddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MACRO_ADDR` reader - Macro address. This field specifies an eFUSE macro."]
pub type MacroAddrR = crate::FieldReader;
#[doc = "Field `MACRO_ADDR` writer - Macro address. This field specifies an eFUSE macro."]
pub type MacroAddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `START` reader - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub fn bit_data(&self) -> BitDataR {
        BitDataR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub fn bit_addr(&self) -> BitAddrR {
        BitAddrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    pub fn byte_addr(&self) -> ByteAddrR {
        ByteAddrR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    pub fn macro_addr(&self) -> MacroAddrR {
        MacroAddrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    #[must_use]
    pub fn bit_data(&mut self) -> BitDataW<CmdSpec> {
        BitDataW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    #[must_use]
    pub fn bit_addr(&mut self) -> BitAddrW<CmdSpec> {
        BitAddrW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    #[must_use]
    pub fn byte_addr(&mut self) -> ByteAddrW<CmdSpec> {
        ByteAddrW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    #[must_use]
    pub fn macro_addr(&mut self) -> MacroAddrW<CmdSpec> {
        MacroAddrW::new(self, 16)
    }
    #[doc = "Bit 31 - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CmdSpec> {
        StartW::new(self, 31)
    }
}
#[doc = "Command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0x01"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0x01;
}
