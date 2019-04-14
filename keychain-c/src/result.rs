use keychain::{ Error as RError };
use std::os::raw::c_char;
use std::error::{ Error as IError };
use std::ffi::{ CStr, CString };

pub trait Ptr<T: ?Sized> {
  unsafe fn as_ref(&self) -> &T;
  unsafe fn free(&mut self);
}

pub trait ArrayPtr<T> {
  unsafe fn as_ref(&self) -> &[T];
  unsafe fn free(&mut self);
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
  WrongPassword = 0,
  NotEnoughData = 1,
  CantCalculateSeedSize = 2,
  DataError = 3,
  EntropyGeneratorError = 4,
  InvalidSeedSize = 5,
  KeyDoesNotExist = 6,
  KeyError = 7,
  KeyPathError = 8,
  MnemonicError = 9
}

#[repr(C)]
pub struct ErrorPtr {
  error_type: ErrorType,
  message: CharPtr
}

impl Ptr<str> for ErrorPtr {
  unsafe fn as_ref(&self) -> &str {
    (&self.message as &Ptr<str>).as_ref()
  }

  unsafe fn free(&mut self) {
    if self.message.is_null() { return; }
    self.message.free();
  }
}

impl ErrorPtr {
  fn error_type(err: &RError) -> ErrorType {
    match err {
      &RError::WrongPassword => ErrorType::WrongPassword,
      &RError::NotEnoughData => ErrorType::NotEnoughData,
      &RError::CantCalculateSeedSize(_, _) => ErrorType::CantCalculateSeedSize,
      &RError::DataError(_) => ErrorType::DataError,
      &RError::EntropyGeneratorError(_) => ErrorType::EntropyGeneratorError,
      &RError::InvalidSeedSize(_) => ErrorType::InvalidSeedSize,
      &RError::KeyDoesNotExist(_) => ErrorType::KeyDoesNotExist,
      &RError::KeyError(_, _) => ErrorType::KeyError,
      &RError::KeyPathError(_) => ErrorType::KeyPathError,
      &RError::MnemonicError(_) => ErrorType::MnemonicError
    }
  }

  pub fn new(err: &RError) -> Self {
    Self {
      error_type: Self::error_type(err),
      message: err.description().to_cstr()
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_error(error: &mut ErrorPtr) {
  error.free();
}

pub trait CResult<T> {
  fn response(&self, val: &mut T, error: &mut ErrorPtr) -> bool;
}

impl<T: Copy> CResult<T> for Result<T, RError> {
  fn response(&self, val: &mut T, error: &mut ErrorPtr) -> bool {
    match self {
      Err(err) => {
        *error = ErrorPtr::new(err);
        false
      },
      Ok(value) => {
        *val = *value;
        true
      }
    }
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataPtr {
  ptr: *const u8,
  len: usize 
}

impl ArrayPtr<u8> for DataPtr {
  unsafe fn as_ref(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr, self.len)
  }

  unsafe fn free(&mut self) {
    if self.ptr.is_null() { return; }
    let _ = Vec::from_raw_parts(self.ptr as *mut u8, self.len, self.len);
    self.ptr = std::ptr::null();
  }
}

impl From<&[u8]> for DataPtr {
  fn from(data: &[u8]) -> Self {
    Vec::from(data).into()
  }
}

impl From<Vec<u8>> for DataPtr {
  fn from(data: Vec<u8>) -> Self {
    let len = data.len();
    let mut slice = data.into_boxed_slice();
    let out = slice.as_mut_ptr();
    std::mem::forget(slice);
    Self { ptr: out, len: len }
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_data(data: &mut DataPtr) {
  data.free();
}

pub type CharPtr = *const c_char;

impl Ptr<str> for CharPtr {
  unsafe fn as_ref(&self) -> &str {
    CStr::from_ptr(*self).to_str().unwrap()
  }

  unsafe fn free(&mut self) {
    let _ = CString::from_raw(*self as *mut c_char);
    *self = std::ptr::null();
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_string(ptr: &mut CharPtr) {
  ptr.free();
}

pub trait ToCString {
  fn to_cstr(&self) -> CharPtr; 
}

impl ToCString for &str {
  fn to_cstr(&self) -> CharPtr {
    CString::new(self.as_bytes()).unwrap().into_raw()
  } 
}

impl ToCString for String {
  fn to_cstr(&self) -> CharPtr {
    CString::new(self.as_bytes()).unwrap().into_raw()
  } 
}