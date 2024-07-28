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

# Buffer most text lines.
{
    if (to_wrap && to_wrap != "\n") {
        to_wrap = to_wrap "\n";
    }
    to_wrap = to_wrap $0;
}

# Wrap any remaining lines at the end.
END {
    flush();
}
