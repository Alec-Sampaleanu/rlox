pub type Value = f32;

#[derive(Debug)]
pub struct ValueArray { values: Vec<Value> }

impl ValueArray {
    pub fn init() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> usize {
        self.values.push(value);
        return self.values.len();
    }

    pub fn get(&self, index: usize) -> &Value {
        match self.values.get(index) {
            Some(val) => val,
            None => panic!()
        }
    }
}