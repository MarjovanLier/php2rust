<?php

declare(strict_types=1);

$total = 0;
foreach ([1, 2, 3] as $item) {
    $total += $item;
}
foreach (['x', 'y'] as $item) {
    echo $item;
}
$item = 'end';
echo ' ', $total, ' ', $item, "\n";
foreach ([5, 6] as $item => $value) {
    echo $item + $value, ' ';
}
foreach ([true] as $value) {
    if ($value) {
        echo "true\n";
    }
}
