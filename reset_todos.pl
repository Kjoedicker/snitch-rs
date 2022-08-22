use 5.30.3;

my $base_file_path = "./test-todos";
my $total_files = 50;

sub compile_program {
    my $compile_command = "cargo build --release";

    say(`$compile_command`);
}

sub exec_program {
    my $exec_command = "./target/release/mesh";

    say(`$exec_command`);
}

sub create_untagged_todos {
    my @range = (1..$total_files);
    foreach(@range) {
        my $filename = "$base_file_path/todo$_";
        my $command = "cp $base_file_path/base.txt $filename.rs";
        my $result = `$command`;
    }
}

sub create_completed_todos {
    my $command = "sed -i \"\" 's/TODO/DONE/g' ./test-todos/*.rs";

    say(`$command`);
}

# Step 1: Compile
compile_program();
# Step 2: Test untagged todos
create_untagged_todos();
exec_program();
# Step 3: Test completed todos
create_completed_todos();
exec_program();