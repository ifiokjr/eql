#[macro_export]
macro_rules! const_join {
  ($ty:ty, $a:expr, $b:expr $(,)*) => {
    &{
      const A: &[$ty] = $a;
      const B: &[$ty] = $b;
      let mut out: [_; { A.len() + B.len() }] = if A.len() == 0 && B.len() == 0 {
        unsafe { std::mem::transmute([0u8; std::mem::size_of::<$ty>() * (A.len() + B.len())]) }
      } else if A.len() == 0 {
        [B[0]; { A.len() + B.len() }]
      } else {
        [A[0]; { A.len() + B.len() }]
      };
      let mut i = 0;
      while i < A.len() {
        out[i] = A[i];
        i += 1;
      }
      i = 0;
      while i < B.len() {
        out[i + A.len()] = B[i];
        i += 1;
      }
      out
    }
  };
}

// pub use const_join;
