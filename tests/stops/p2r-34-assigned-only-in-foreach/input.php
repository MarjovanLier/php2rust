<?php

declare(strict_types=1);

/** @param list<string> $names */
function lastName(array $names): string
{
    foreach ($names as $name) {
        $last = $name;
    }
    return $last;
}

echo lastName(['a', 'b']), "\n";
