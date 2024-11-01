pub fn loop_template(iterator_variable: String, n_iterations: String, nr_loops: String) -> String {
    // means that we have a variable as a "limit"
    if n_iterations.chars().all(|f| !f.is_numeric()) {
        return format!(
            "
        %x =w phi @start_{nr_loops} {n_iterations}, @loop_{nr_loops} %x1
        %{iterator_variable} =w sub %x, 1
        jnz %{iterator_variable}, @loop_{nr_loops}, @end_{nr_loops}
        @end_{nr_loops}
        "
        );
    }
    format!(
        "%x =w phi @start_{nr_loops} {n_iterations}, @loop_{nr_loops} %x1
        %{iterator_variable} =w sub %x, 1
        jnz %{iterator_variable}, @loop_{nr_loops}, @end_{nr_loops}
        @end_{nr_loops}
        "
    )
}
