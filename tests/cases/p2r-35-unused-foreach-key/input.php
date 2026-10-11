<?php

declare(strict_types=1);

$prices = [4, 6];
$sum = 0;
foreach ($prices as $key => $price) {
    $sum += $price;
}
$count = 0;
foreach ($prices as $price) {
    $count++;
}
$positions = 0;
$letters = ['a', 'b', 'c'];
foreach ($letters as $position => $letter) {
    $positions += $position;
}
echo $sum, ' ', $count, ' ', $positions, "\n";
