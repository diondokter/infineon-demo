#[doc = "Register `AES_CTL` reader"]
pub type R = crate::R<AesCtlSpec>;
#[doc = "Register `AES_CTL` writer"]
pub type W = crate::W<AesCtlSpec>;
#[doc = "AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeySize {
    #[doc = "0: N/A"]
    Aes128 = 0,
    #[doc = "1: N/A"]
    Aes192 = 1,
    #[doc = "2: N/A"]
    Aes256 = 2,
}
impl From<KeySize> for u8 {
    #[inline(always)]
    fn from(variant: KeySize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KeySize {
    type Ux = u8;
}
impl crate::IsEnum for KeySize {}
#[doc = "Field `KEY_SIZE` reader - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
pub type KeySizeR = crate::FieldReader<KeySize>;
impl KeySizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KeySize> {
        match self.bits {
            0 => Some(KeySize::Aes128),
            1 => Some(KeySize::Aes192),
            2 => Some(KeySize::Aes256),
            _ => None,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KeySize::Aes128
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KeySize::Aes192
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KeySize::Aes256
    }
}
#[doc = "Field `KEY_SIZE` writer - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
pub type KeySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, KeySize>;
impl<'a, REG> KeySizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(KeySize::Aes128)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut crate::W<REG> {
        self.variant(KeySize::Aes192)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(KeySize::Aes256)
    }
}
impl R {
    #[doc = "Bits 0:1 - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
    #[inline(always)]
    #[must_use]
    pub fn key_size(&mut self) -> KeySizeW<AesCtlSpec> {
        KeySizeW::new(self, 0)
    }
}
#[doc = "AES control\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCtlSpec;
impl crate::RegisterSpec for AesCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ctl::R`](R) reader structure"]
impl crate::Readable for AesCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_ctl::W`](W) writer structure"]
impl crate::Writable for AesCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_CTL to value 0"]
impl crate::Resettable for AesCtlSpec {
    const RESET_VALUE: u32 = 0;
}
