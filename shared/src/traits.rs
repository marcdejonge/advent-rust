pub auto trait NotEq {}
impl<X> !NotEq for (X, X) {}
