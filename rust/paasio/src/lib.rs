use std::io::{Read, Result, Write};

// pass through the data and get rid of the phantomData :/
// - The total number of bytes read/written. -> bytes for the ReadStats Struct
// - The total number of read/write operations. -> reads for the ReadStats Struc
pub struct ReadStats<R> {
    wrapped: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped: _wrapped,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // upon call, a read has been done
        self.reads += 1;
        let result = self.wrapped.read(buf);

        let bytes_read = match result {
            Ok(n) => n,
            Err(e) => panic!("Failed to read, {:?}", e),
        };

        self.bytes += bytes_read;
        return result;
    }
}

// pass through the data and get rid of the phantomData :/
// - The total number of bytes read/written. -> bytes for the WriteStats Struct
// - The total number of read/write operations. -> reads for the WriteStats Struc
pub struct WriteStats<W> {
    wrapped: W,
    bytes: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped: _wrapped,
            bytes: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        let result = self.wrapped.write(buf);

        let bytes_written = match result {
            Ok(n) => n,
            Err(e) => panic!("Failed to write, {:?}", e),
        };

        self.bytes += bytes_written;
        return result;
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
