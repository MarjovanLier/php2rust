<?php

declare(strict_types=1);

$values = [4, 7];
foreach ($values as $key => $value) {
    echo $value, ' ';
}
echo $key, "\n";
foreach ($values as $value) {
    echo $value, ' ';
}
echo $value, "\n";
$x = 0;
$i = 0;
while ($i < 2) {
    echo $x, ' ';
    foreach ([5, 6] as $x) {
        echo $x;
    }
    $i++;
}
