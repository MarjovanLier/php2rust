<?php

declare(strict_types=1);

/** @param list<int> $values */
function total(array $values): int
{
    $sum = 0;
    foreach ($values as $value) {
        $sum += $value;
    }
    return $sum;
}

$numbers = [3, 8, 5, 9];
foreach ($numbers as $n) {
    if ($n === 8) {
        continue;
    }
    if ($n === 9) {
        break;
    }
    $n *= 2;
    echo $n, ' ';
}
echo total($numbers), "\n";
$names = ['Ann', 'Bob'];
$last = '';
foreach ($names as $name) {
    echo "Hi $name\n";
    $last = $name;
}
echo $last, ' ', count($names), "\n";
$flags = [true, false, true];
$yes = 0;
foreach ($flags as $flag) {
    if ($flag) {
        $yes++;
    }
}
echo $yes, "\n";
foreach ([[1, 2], [3]] as $row) {
    echo count($row), ' ';
}
$words = ['end'];
foreach ($words as $word):
    $word .= '!';
    echo $word, "\n";
endforeach;
