
cargo build -r

pass_count=0
fail_count=0

echo "┌──────────────┬────────┬────────────────────────────────────────────────────────────────────────────────────────────┐"
echo "│ file         │ result │ output                                                                                     │"
echo "├──────────────┼────────┼────────────────────────────────────────────────────────────────────────────────────────────┤"
for file in $(ls ./tests/files | sort -sV) ; do
    output=$(./target/release/json-parser ./tests/files/$file)
    result=""
    if [[ ($file =~ ^pass && $output == "ok") || ($file =~ ^fail && $output != "ok")]]; then
        result="pass"
        pass_count=$((pass_count+1))
    else
        result="fail"
        fail_count=$((fail_count+1))
    fi
    awk -v f="$file" -v r="$result" -v o="$output" 'BEGIN{print "│ " sprintf("%-12s", f) " │ " r "   │ " sprintf("%-90s", o) " │"}'
done
echo "└──────────────┴────────┴────────────────────────────────────────────────────────────────────────────────────────────┘"

echo "pass: $pass_count"
echo "fail: $fail_count"