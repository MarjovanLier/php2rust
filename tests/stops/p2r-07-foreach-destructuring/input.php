<?php

declare(strict_types=1);

$pairs = [[1, 2], [3, 4]];
foreach ($pairs as [$a, $b]) {
    echo $a + $b, ' ';
}
echo count($pairs), "\n";
