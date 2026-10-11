<?php

declare(strict_types=1);

/** @param list<string> $words */
function numbered(array $words): void
{
    foreach ($words as $index => $word) {
        if ($index === 2) {
            break;
        }
        $index++;
        echo $index, ". $word\n";
    }
}

$scores = [70, 95, 80];
$best = 0;
$bestAt = -1;
foreach ($scores as $i => $score) {
    if ($score > $best) {
        $best = $score;
        $bestAt = $i;
    }
}
echo $bestAt, ' ', $best, ' ', count($scores), "\n";
numbered(['a', 'b', 'c']);
$grid = [[1, 2], [3, 4, 5]];
foreach ($grid as $row => $cells) {
    foreach ($cells as $column => $cell) {
        echo $row * 10 + $column, '=', $cell, ' ';
    }
}
echo "\n";
