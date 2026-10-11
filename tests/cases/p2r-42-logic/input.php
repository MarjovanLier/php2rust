<?php

declare(strict_types=1);

function check(int $n, bool $result): bool
{
    echo $n;
    return $result;
}

if (check(1, false) && check(2, true)) {
    echo 'A';
}
if (check(3, true) || check(4, true)) {
    echo 'B';
}
if (!check(5, false)) {
    echo 'C';
}
if (!(check(6, true) && check(7, false)) || check(8, true)) {
    echo 'D';
}
if (check(9, false) || check(10, true) && check(11, false)) {
    echo 'E';
}
if ((check(12, false) || check(13, true)) && check(14, true)) {
    echo 'F';
}
$ready = check(15, true) && !check(16, false);
if ($ready) {
    echo 'G';
}
echo "\n";
