<?php

declare(strict_types=1);

function below(int $n, int $limit): bool
{
    return $n < $limit;
}

$i = 0;
$sum = 0;
while ($i < 10) {
    $i++;
    if ($i % 2 === 0) {
        continue;
    }
    if ($i > 7) {
        break;
    }
    $square = $i * $i;
    $sum += $square;
    echo $square, ' ';
}
echo $sum, "\n";

$base = 100;
$k = 5;
while (below($k, 8)):
    echo $base % $k, ' ';
    $k++;
endwhile;
echo "\n";

$d = 0;
while ($d < 3) $d++;
echo 7 % $d, "\n";
