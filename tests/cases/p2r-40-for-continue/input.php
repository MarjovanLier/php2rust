<?php

declare(strict_types=1);

function countDown(int $from): void
{
    for ($k = $from; $k > 0; $k--) echo $k;
    echo "\n";
}

for ($i = 0, $j = 10; $i < $j; $i++, $j--) {
    if ($i % 3 === 0) {
        continue;
    }
    echo $i, ':', $j, ' ';
}
echo "\n";

$total = 0;
for ($n = 1; $n <= 5; $n++):
    if ($n === 2) continue;
    $total += $n;
endfor;
echo $total, "\n";

countDown(3);
