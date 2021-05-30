
#[derive(Debug)]
pub struct Rom {
    inner: Vec<u8>,
}

impl Rom {
    pub fn new(v: Vec<u8>) -> Rom {
        Rom {
            inner: v.clone(),
        }
    }
}
