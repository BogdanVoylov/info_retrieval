pub trait GET<Idx, Output> {
    fn get(&self, idx:Idx) -> Output;
}

pub trait PUT<Input> {
    fn put(&mut self, item: Input);
}

pub trait CRUD<Idx, Input, Output> : GET<Idx,Output> + PUT<Input> {}

