#[doc = "Register `CRYPTO_CMD` reader"]
pub type R = crate::R<CryptoCmdSpec>;
#[doc = "Register `CRYPTO_CMD` writer"]
pub type W = crate::W<CryptoCmdSpec>;
#[doc = "Field `START` reader - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CryptoCmdSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "Cryptography Command\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoCmdSpec;
impl crate::RegisterSpec for CryptoCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_cmd::R`](R) reader structure"]
impl crate::Readable for CryptoCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_cmd::W`](W) writer structure"]
impl crate::Writable for CryptoCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_CMD to value 0"]
impl crate::Resettable for CryptoCmdSpec {
    const RESET_VALUE: u32 = 0;
}
