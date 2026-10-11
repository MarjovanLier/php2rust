<?php

declare(strict_types=1);

/** @param list<string> $names */
function first(array $names): string
{
    return $names[0];
}

$names = ['Ann', 'Bob'];
$second = $names[1];
$names[] = 'Cy';
$copy = $names;
$copy[] = 'Di';
$i = count($names) - 1;
$last = $names[$i];
echo count($names), ' ', count($copy), ' ', $second, ' ', $last, "\n";
echo first($copy), ' ', first(['Eve']), "\n";
