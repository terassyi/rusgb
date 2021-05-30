
#[derive(Debug)]
pub struct Ram {
    inner: Vec<u8>
}

impl Ram {
    pub fn new(v: Vec<u8>) -> Ram {
        Ram {
            inner: v.clone(),
        }
    }
}
