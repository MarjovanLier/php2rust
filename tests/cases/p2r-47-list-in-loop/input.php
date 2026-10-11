<?php

declare(strict_types=1);

$a = [3, 8];
$found = [];
$i = 0;
$total = 0;
while ($i < count($a)) {
    if ($a[$i] > 5) {
        $total += $a[$i];
        $found[] = $i;
    } else {
        $a[] = $a[$i] * 10;
    }
    $i++;
}
echo $total, ' ', count($a), ' ', count($found), ' ', $found[0], "\n";
if (count($found) < 3) {
    echo "few\n";
}
