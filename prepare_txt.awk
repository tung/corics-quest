# Wrap text through 'fmt', except certain lines.

# Wrap buffered lines.
function flush() {
    if (to_wrap && to_wrap != "\n") {
        fmt = "fmt";
        print to_wrap | fmt;
        close(fmt);
        to_wrap = "\n";
    }
}

BEGIN {
    skip_blank_line = 0;
}

# Print list lines as-is.
/^ - / {
    flush();
    print;
    next;
}

# Print reference link lines as-is.
/^[[a-z0-9\-]+\]:/ {
    flush();
    print;
    next;
}

# Skip image lines, as well as any following blank line.
/^!\[/ {
    skip_blank_line = 1;
    next;
}

# Skip blank line if requested.
/^$/ && skip_blank_line == 1 {
    skip_blank_line = 0;
    next;
}

# Buffer most text lines.
{
    skip_blank_line = 0;
    if (to_wrap && to_wrap != "\n") {
        to_wrap = to_wrap "\n";
    }
    to_wrap = to_wrap $0;
}

# Wrap any remaining lines at the end.
END {
    flush();
}
