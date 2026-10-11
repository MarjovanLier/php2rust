<?php

declare(strict_types=1);

$queue = [1, 2];
foreach ($queue as $item) {
    $queue[] = $item * 10;
}
$names = ['a'];
foreach ($names as $index => $name) {
    if ($index === 0) {
        $names = [$name . $name];
    }
}
echo count($queue), ' ', count($names), "\n";
