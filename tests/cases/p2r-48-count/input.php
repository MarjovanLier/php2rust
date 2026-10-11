<?php

declare(strict_types=1);

$grid = [[1, 2], [3]];
$row = [4, 5, 6];
echo count($row), ' ', count($grid) * 10, ' ', COUNT($grid[0]) + 1, ' ', -count($row), "\n";
