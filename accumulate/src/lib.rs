// pub fn map<F, In, Out>(input: Vec<In>, function: F) -> Vec<Out> 
// where 
//     F: FnMut(In) -> Out, 
// {
//     input.into_iter().map(function).collect()
// }


// Without std.
pub fn map<F, T, U>(input: Vec<T>, mut function: F) -> Vec<U> 
where 
    F: FnMut(T) -> U, 
{
    let mut res = Vec::with_capacity(input.len());
    for x in input.into_iter() {
        res.push(function(x));
    }
    res
}