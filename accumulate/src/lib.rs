pub fn map<T, O, F: FnMut(T) -> O>(input: Vec<T>, mut function: F) -> Vec<O> {
    let mut output = Vec::with_capacity(input.len());
    for x in input {
        output.push(function(x));
    }
    output
}
