<?php

declare(strict_types=1);

$flags = [true, false];
$flags[] = true;
$on = 0;
$i = 0;
while ($i < count($flags)) {
    if ($flags[$i]) {
        $on++;
    }
    $i++;
}
echo $on, ' ', count($flags), "\n";
