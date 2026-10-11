<?php

declare(strict_types=1);

$item = 'start';
echo $item, ' ';
$total = 0;
foreach ([1, 2, 3] as $item) {
    $total += $item;
}
$item = 'end';
echo $total, ' ', $item, "\n";
$index = 'none';
echo $index, ' ';
foreach (['a', 'b'] as $index => $letter) {
    echo $index, $letter;
}
echo "\n";
