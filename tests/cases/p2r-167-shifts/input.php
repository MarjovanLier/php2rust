<?php

declare(strict_types=1);

/**
 * @param list<int> $xs
 */
function packed(array $xs, int $n): int
{
    $flags = $n << 3;
    $flags >>= 1;
    $flags <<= 2;
    return (count($xs) << 2) + ($flags >> 1) + (-$n >> 2);
}

final class Bits
{
    public function __construct(public int $mask) {}

    public function grow(): void
    {
        $this->mask <<= 4;
        $this->mask >>= 1;
    }
}

$min = 1 << 63;
$big = 1 << 40;
$bits = new Bits(3);
$bits->grow();
echo packed([1, 2, 3], 5), "\n";
echo $min, " ", $big, " ", -9 >> 1, " ", 5 >> 63, "\n";
echo $bits->mask, "\n";
echo 1 + 2 << 3, " ", 7 << 2 >> 1, "\n";
$c = 1 << 2 < 5;
if ($c) {
    echo "less\n";
}
