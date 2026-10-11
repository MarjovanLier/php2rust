<?php

declare(strict_types=1);

function value(): int
{
    return 1;
}

foreach ([1] as $value) {
    echo $value;
}
$myItem = 2;
echo $myItem;
foreach ([3] as $my_item) {
    echo $my_item, value(), "\n";
}
