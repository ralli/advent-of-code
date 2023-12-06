my $filename = "day-6.txt";
open FILE, "<$filename" or die "cannot open $filename: $!\n";

my $times = <FILE>;
my $distances = <FILE>;

my @times = $times =~ /(\d+)/g;
my @distances = $distances =~ /(\d+)/g;

my $num_elems =  @times;
my @ans;
for (my $i = 0; $i < $num_elems; $i++) {
	my $time = @times[$i];
	my $distance = @distances[$i];
	my $n = 0;
	for (my $t = 0; $t < $time; $t++) {
		$d = ($time - $t) * $t;
		if ($d > $distance) {
			$n++;
		}
	}
	push @ans, $n;
}

my $ans = 1;
foreach (@ans) {
	$ans *= $_;
}
print $ans, "\n";

$time = join("", @times);
$distance = join("", @distances);
my $n = 0;
for (my $t = 0; $t < $time; $t++) {
	$d = ($time - $t) * $t;
	if ($d > $distance) {
		$n++;
	}
}
print $n, "\n";
