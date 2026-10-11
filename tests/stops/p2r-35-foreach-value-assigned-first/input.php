<?php

declare(strict_types=1);

foreach ([1, 2] as $key => $value) {
    $value = 5;
    $key = 1;
    echo $value + $key, "\n";
}
