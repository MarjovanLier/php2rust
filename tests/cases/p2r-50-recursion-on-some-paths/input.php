<?php

declare(strict_types=1);

function down(int $n): bool
{
    echo $n;
    return $n > 0 && down($n - 1);
}

function seek(int $n): int
{
    while ($n > 3) {
        if ($n % 2 === 0) {
            return seek($n - 1);
        }
        $n -= 3;
    }
    return $n;
}

function countTo(int $n, int $limit): void
{
    for (; $n <= $limit; $n++) {
        if ($n === $limit) {
            echo $n, "\n";
            break;
        }
        echo $n, ' ';
    }
}

function halve(int $n): int
{
    if ($n > 1) {
        return halve(intdiv($n, 2)) + 1;
    }
    return 0;
}

if (down(3)) {
    echo " down\n";
}
echo seek(10), ' ', seek(11), ' ', halve(40), "\n";
countTo(4, 7);
