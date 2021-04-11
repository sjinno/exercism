pub fn map<F, In, Out>(input: Vec<In>, function: F) -> Vec<Out>
where
    F: FnMut(In) -> Out,
{
    input.into_iter().map(function).collect()
}
