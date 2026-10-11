<?php

declare(strict_types=1);

function three(): int
{
    return 3;
}

$divisor = 0;
echo $divisor, ' ';
$divisor = three();
echo 10 % $divisor, ' ';
$big = 9223372036854775807;
$big = $big - three();
echo $big + 2, "\n";
