fn main() {
    let context_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop, 
    bedroom window, public-house, and 
    dark square is a picture 
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek 
    through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut context: Vec<Vec<(usize, String)>> = vec![];

    // First pass, just capture lines where needle matches
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * context_lines + 1);
            context.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_context = (i, line_as_string);
                context[j].push(local_context);
            }
        }
    }

    for local_context in context.iter() {
        for &(i, ref line) in local_context.iter() {
            let line_number = i + 1;
            println!("{}: {}", line_number, line);
        }
    }
}