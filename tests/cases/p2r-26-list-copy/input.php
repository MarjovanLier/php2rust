<?php

declare(strict_types=1);

$a = [1, 2];
$b = $a;
$b[] = 3;
echo count($a), ' ', count($b), ' ';
$grid = [$a, $b];
$first = $grid[0];
$first[] = 9;
echo count($grid[0]), ' ', count($first), ' ';
$grid[] = $b;
echo count($grid), "\n";
