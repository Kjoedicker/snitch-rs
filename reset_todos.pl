my $base_file_path = "./test-todos";

my $total_files = 50;

my @range = (1..$total_files);
foreach(@range) {
    my $filename = "$base_file_path/todo$_";
    my $command = "cp $base_file_path/base.txt $filename.rs";
    my $result = `$command`;
}
