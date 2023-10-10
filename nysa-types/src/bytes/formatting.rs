use super::FixedBytes;
use core::{fmt, str};

impl<const N: usize> fmt::Display for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If the alternate flag is NOT set, we write the full hex.
        if N <= 4 || !f.alternate() {
            return self.fmt_hex::<false>(f, true);
        }

        // If the alternate flag is set, we use middle-out compression.
        const SEP_LEN: usize = '…'.len_utf8();
        let mut buf = [0; 2 + 4 + SEP_LEN + 4];
        buf[0] = b'0';
        buf[1] = b'x';
        const_hex::encode_to_slice(&self.0[0..2], &mut buf[2..6]).unwrap();
        '…'.encode_utf8(&mut buf[6..]);
        const_hex::encode_to_slice(&self.0[N - 2..N], &mut buf[6 + SEP_LEN..]).unwrap();

        // SAFETY: always valid UTF-8
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}

impl<const N: usize> fmt::LowerHex for FixedBytes<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<false>(f, f.alternate())
    }
}

impl<const N: usize> fmt::UpperHex for FixedBytes<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<true>(f, f.alternate())
    }
}

impl<const N: usize> fmt::Debug for FixedBytes<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<false>(f, true)
    }
}

impl<const N: usize> FixedBytes<N> {
    fn fmt_hex<const UPPER: bool>(&self, f: &mut fmt::Formatter<'_>, prefix: bool) -> fmt::Result {
        let mut buf = const_hex::Buffer::<N, true>::new();
        let s = if UPPER {
            buf.format_upper(self)
        } else {
            buf.format(self)
        };
        // SAFETY: The buffer is guaranteed to be at least 2 bytes in length.
        f.write_str(unsafe { s.get_unchecked((!prefix as usize) * 2..) })
    }
}
