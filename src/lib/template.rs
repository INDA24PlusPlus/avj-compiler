pub fn loop_template(iterator_variable: String, n_iterations: String) -> String {
    // means that we have a variable as a "limit"
    if n_iterations.chars().all(|f| !f.is_numeric()) {
        return format!(
            "@start
@loop
        %x =w phi @start %{n_iterations}, @loop %{iterator_variable}
        %{iterator_variable} =w sub %x, 1
        jnz %{iterator_variable}, @loop, @end
@end
        ret"
        );
    }
    format!(
        "@start
@loop
        %x =w phi @start {n_iterations}, @loop %x1
        %{iterator_variable} =w sub %x, 1
        jnz %{iterator_variable}, @loop, @end
@end
        ret"
    )
}
