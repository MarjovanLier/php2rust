<?php

declare(strict_types=1);

$values = [1, 2];
foreach ($values as &$value) {
    $value *= 2;
}
echo count($values), "\n";
