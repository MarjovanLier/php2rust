<?php

declare(strict_types=1);

$scores = [7, 3];
$i = 1;
echo $scores[0], ' ', $scores[$i], ' ';
$scores[] = 10;
$last = count($scores) - 1;
$grid = [[1, 2], [3]];
echo $scores[$last], ' ', $scores[$last - 1], ' ', $grid[1][0], "\n";
