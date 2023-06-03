
cargo build -r

# iterate through all the files in test_files/full
echo "Running fail tests..."
for file in ./tests/files/fail*; do
    # run the program on the file
    echo $file
    ./target/release/json-parser $file 
    echo " "
    # compare the output to the expected output
    # diff tests/test_files/full/output.txt tests/test_files/full/expected_output.txt
    # if [ $? -eq 0 ]; then
    #     echo "Test passed"
    # else
    #     echo "Test failed"
    # fi
done

echo "Running pass tests..."
for file in ./tests/files/pass*; do
    # run the program on the file
    echo $file
    ./target/release/json-parser $file
    echo " "
    # compare the output to the expected output
    # diff tests/test_files/full/output.txt tests/test_files/full/expected_output.txt
    # if [ $? -eq 0 ]; then
    #     echo "Test passed"
    # else
    #     echo "Test failed"
    # fi
done