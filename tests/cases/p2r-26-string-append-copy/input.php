<?php

declare(strict_types=1);

$names = [];
$name = 'Ann';
$names[] = $name;
$name .= 'a';
$names[] = $name;
echo $names[0], ' ', $names[1], ' ', $name, "\n";
