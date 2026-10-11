<?php

declare(strict_types=1);

$count = 0;
for ($x = 0.5; $x < 3.0; $x++) {
    $count++;
}
for ($y = 2.5; $y > 0.0; --$y) {
    $count += 10;
}
echo $count, "\n";
