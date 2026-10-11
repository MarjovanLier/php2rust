<?php

declare(strict_types=1);

$s = 'ab';
$s .= $s;
$s .= "-$s";
$s .= 5;
$n = 3;
$s .= $n . '{}';
echo $s, "\n";
