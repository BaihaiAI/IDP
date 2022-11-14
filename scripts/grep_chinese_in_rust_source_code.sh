if grep --perl-regexp '[\p{Han}]' --include "*.rs" --line-number ./crates -r; then
    echo "error: found chinese in rs source file break our code style" 1>&2
    exit 1
fi