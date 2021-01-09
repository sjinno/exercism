// use std::io::{Read, Result, Write};

// // #[derive(Clone, Copy)]
// pub struct ReadStats<R>(R);

// impl<R: Read + Copy> ReadStats<R> {
//     // _wrapped is ignored because R is not bounded on Debug or Display and therefore
//     // can't be passed through format!(). For actual implementation you will likely
//     // wish to remove the leading underscore so the variable is not ignored.
//     pub fn new(wrapped: R) -> ReadStats<R> {
//         ReadStats(wrapped)
//     }

//     pub fn get_ref(&self) -> &R {
//         &self.0
//     }

//     pub fn bytes_through(&self) -> usize {
//         unimplemented!()
//     }

//     pub fn reads(&self) -> usize {
//         let mut contents = self.0;
//         let mut buffer = String::new();
//         contents.read_to_string(&mut buffer).unwrap()
//     }
// }

// impl<R: Read> Read for ReadStats<R> {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         self.0.read_to_end(&mut Vec::from(buf))
//     }
// }

// pub struct WriteStats<W>(W);

// impl<W: Write> WriteStats<W> {
//     // _wrapped is ignored because W is not bounded on Debug or Display and therefore
//     // can't be passed through format!(). For actual implementation you will likely
//     // wish to remove the leading underscore so the variable is not ignored.
//     pub fn new(wrapped: W) -> WriteStats<W> {
//         WriteStats(wrapped)
//     }

//     pub fn get_ref(&self) -> &W {
//         unimplemented!()
//     }

//     pub fn bytes_through(&self) -> usize {
//         unimplemented!()
//     }

//     pub fn writes(&self) -> usize {
//         unimplemented!()
//     }
// }

// impl<W: Write> Write for WriteStats<W> {
//     fn write(&mut self, buf: &[u8]) -> Result<usize> {
//         unimplemented!("Collect statistics about this call writing {:?}", buf)
//     }

//     fn flush(&mut self) -> Result<()> {
//         unimplemented!()
//     }
// }

// use std::io::{Read, Result, Write};

// pub type ReadStats<T> = IoStats<T>;
// pub type WriteStats<T> = IoStats<T>;

// pub struct IoStats<T> {
//     wrapped: T,
//     bytes_through: usize,
//     writes: usize,
//     reads: usize,
// }

// impl<T> IoStats<T> {
//     pub fn new(wrapped: T) -> IoStats<T> {
//         IoStats {
//             wrapped,
//             bytes_through: 0,
//             writes: 0,
//             reads: 0,
//         }
//     }

//     pub fn get_ref(&self) -> &T {
//         &self.wrapped
//     }

//     pub fn bytes_through(&self) -> usize {
//         self.bytes_through
//     }
// }

// impl<T: Read> IoStats<T> {
//     pub fn reads(&self) -> usize {
//         self.reads
//     }
// }

// impl<T: Write> IoStats<T> {
//     pub fn writes(&self) -> usize {
//         self.writes
//     }
// }

// impl<T: Read> Read for IoStats<T> {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         self.reads += 1;
//         self.wrapped.read(buf).map(|bytes| {
//             self.bytes_through += bytes;
//             bytes
//         })
//     }
// }

// impl<T: Write> Write for IoStats<T> {
//     fn write(&mut self, buf: &[u8]) -> Result<usize> {
//         self.writes += 1;
//         self.wrapped.write(buf).map(|bytes| {
//             self.bytes_through += bytes;
//             bytes
//         })
//     }

//     fn flush(&mut self) -> Result<()> {
//         self.wrapped.flush()
//     }
// }

use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes: usize,
    ops: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped: wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.wrapped.read(buf)?;
        self.bytes += bytes.clone();
        self.ops += 1;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes: usize,
    ops: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped: wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.wrapped.write(buf)?;
        self.ops += 1;
        self.bytes += bytes.clone();
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
